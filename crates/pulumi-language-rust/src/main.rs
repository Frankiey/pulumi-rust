use std::net::SocketAddr;
use std::process::Stdio;

use anyhow::Result;
use tokio::process::Command;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tonic::{Request, Response, Status};
use tracing::{error, info};

use pulumi_proto::pulumirpc::{
    self, language_runtime_server::LanguageRuntimeServer, AboutRequest, AboutResponse,
    GeneratePackageRequest, GeneratePackageResponse, GenerateProgramRequest,
    GenerateProgramResponse, GenerateProjectRequest, GenerateProjectResponse,
    GetProgramDependenciesRequest, GetProgramDependenciesResponse, GetRequiredPackagesRequest,
    GetRequiredPackagesResponse, GetRequiredPluginsRequest, GetRequiredPluginsResponse,
    InstallDependenciesRequest, InstallDependenciesResponse, LanguageHandshakeRequest,
    LanguageHandshakeResponse, LinkRequest, LinkResponse, PackRequest, PackResponse, PluginInfo,
    RunPluginRequest, RunPluginResponse, RunRequest, RunResponse, RuntimeOptionsRequest,
    RuntimeOptionsResponse, TemplateRequest, TemplateResponse,
};

struct RustLanguageHost {
    engine_address: Mutex<String>,
}

impl RustLanguageHost {
    fn new() -> Self {
        Self {
            engine_address: Mutex::new(String::new()),
        }
    }
}

#[tonic::async_trait]
impl pulumirpc::language_runtime_server::LanguageRuntime for RustLanguageHost {
    async fn handshake(
        &self,
        request: Request<LanguageHandshakeRequest>,
    ) -> Result<Response<LanguageHandshakeResponse>, Status> {
        let req = request.into_inner();
        info!("Handshake: engine_address={}", req.engine_address);
        *self.engine_address.lock().await = req.engine_address;
        Ok(Response::new(LanguageHandshakeResponse {}))
    }

    async fn get_required_plugins(
        &self,
        _request: Request<GetRequiredPluginsRequest>,
    ) -> Result<Response<GetRequiredPluginsResponse>, Status> {
        Ok(Response::new(GetRequiredPluginsResponse {
            plugins: vec![],
        }))
    }

    async fn get_required_packages(
        &self,
        request: Request<GetRequiredPackagesRequest>,
    ) -> Result<Response<GetRequiredPackagesResponse>, Status> {
        let req = request.into_inner();
        let work_dir = req
            .info
            .as_ref()
            .map(|i| i.program_directory.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or(".");

        let cargo_path = std::path::Path::new(work_dir).join("Cargo.toml");
        let contents = match std::fs::read_to_string(&cargo_path) {
            Ok(c) => c,
            Err(e) => {
                info!("No Cargo.toml found at {}: {e}", cargo_path.display());
                return Ok(Response::new(GetRequiredPackagesResponse {
                    packages: vec![],
                }));
            }
        };

        let doc: toml::Table = contents
            .parse()
            .map_err(|e| Status::internal(format!("failed to parse Cargo.toml: {e}")))?;

        let mut packages = vec![];

        // Scan [dependencies] for pulumi-* crates
        if let Some(deps) = doc.get("dependencies").and_then(|d| d.as_table()) {
            for (name, value) in deps {
                if let Some(provider) = crate_to_provider(name) {
                    let version = match value {
                        toml::Value::String(v) => v.clone(),
                        toml::Value::Table(t) => t
                            .get("version")
                            .and_then(|v| v.as_str())
                            .unwrap_or("0.0.0")
                            .to_string(),
                        _ => "0.0.0".to_string(),
                    };
                    packages.push(pulumi_proto::pulumirpc::PackageDependency {
                        name: provider,
                        kind: "resource".to_string(),
                        version,
                        server: String::new(),
                        checksums: Default::default(),
                        parameterization: None,
                    });
                }
            }
        }

        info!("GetRequiredPackages: found {} packages", packages.len());
        Ok(Response::new(GetRequiredPackagesResponse { packages }))
    }

    async fn run(&self, request: Request<RunRequest>) -> Result<Response<RunResponse>, Status> {
        let req = request.into_inner();

        #[allow(deprecated)]
        let (work_dir, entry_point) = if let Some(info) = &req.info {
            let dir = if info.program_directory.is_empty() {
                req.pwd.clone()
            } else {
                info.program_directory.clone()
            };
            let entry = if info.entry_point.is_empty() || info.entry_point == "." {
                ".".to_string()
            } else {
                info.entry_point.clone()
            };
            (dir, entry)
        } else {
            (req.pwd.clone(), req.program.clone())
        };

        info!("Run: entry_point={entry_point}, pwd={work_dir}");

        // Resolve the binary path
        let binary = match resolve_binary(&work_dir, &entry_point).await {
            Ok(b) => b,
            Err(e) => {
                return Ok(Response::new(RunResponse {
                    error: format!("failed to resolve binary: {e}"),
                    bail: false,
                }));
            }
        };

        info!("Run: resolved binary={binary}");

        let monitor_address = &req.monitor_address;
        let engine_address = self.engine_address.lock().await.clone();
        let project = &req.project;
        let stack = &req.stack;

        let mut cmd = Command::new(&binary);
        cmd.current_dir(&work_dir)
            .env("PULUMI_MONITOR", monitor_address)
            .env("PULUMI_ENGINE", &engine_address)
            .env("PULUMI_PROJECT", project)
            .env("PULUMI_STACK", stack)
            .env("PULUMI_PARALLEL", req.parallel.to_string())
            .env("PULUMI_DRY_RUN", req.dry_run.to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        for (k, v) in &req.config {
            cmd.env(format!("PULUMI_CONFIG_{k}"), v);
        }

        let result = cmd.status().await;

        match result {
            Ok(status) => {
                if status.success() {
                    Ok(Response::new(RunResponse {
                        error: String::new(),
                        bail: false,
                    }))
                } else {
                    let code = status.code().unwrap_or(-1);
                    error!("Program exited with code {code}");
                    Ok(Response::new(RunResponse {
                        error: format!("program exited with code {code}"),
                        bail: false,
                    }))
                }
            }
            Err(e) => {
                error!("Failed to spawn program: {e}");
                Ok(Response::new(RunResponse {
                    error: format!("failed to run program: {e}"),
                    bail: false,
                }))
            }
        }
    }

    async fn get_plugin_info(&self, _request: Request<()>) -> Result<Response<PluginInfo>, Status> {
        #[allow(clippy::needless_update)]
        Ok(Response::new(PluginInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            ..Default::default()
        }))
    }

    type InstallDependenciesStream =
        tokio_stream::wrappers::ReceiverStream<Result<InstallDependenciesResponse, Status>>;

    async fn install_dependencies(
        &self,
        request: Request<InstallDependenciesRequest>,
    ) -> Result<Response<Self::InstallDependenciesStream>, Status> {
        let req = request.into_inner();

        #[allow(deprecated)]
        let work_dir = if let Some(info) = &req.info {
            if info.program_directory.is_empty() {
                req.directory.clone()
            } else {
                info.program_directory.clone()
            }
        } else {
            req.directory.clone()
        };

        info!("InstallDependencies: directory={work_dir}");

        let (tx, rx) = tokio::sync::mpsc::channel(64);

        tokio::spawn(async move {
            let mut cmd = Command::new("cargo");
            cmd.arg("build")
                .arg("--release")
                .current_dir(&work_dir)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped());

            let mut child = match cmd.spawn() {
                Ok(c) => c,
                Err(e) => {
                    let msg = format!("failed to spawn cargo build: {e}\n");
                    let _ = tx
                        .send(Ok(InstallDependenciesResponse {
                            stdout: vec![],
                            stderr: msg.into_bytes(),
                        }))
                        .await;
                    return;
                }
            };

            // Stream stderr (cargo sends most output there)
            let stderr = child.stderr.take();
            let tx_err = tx.clone();
            let stderr_task = tokio::spawn(async move {
                if let Some(stderr) = stderr {
                    use tokio::io::AsyncBufReadExt;
                    let reader = tokio::io::BufReader::new(stderr);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        let mut bytes = line.into_bytes();
                        bytes.push(b'\n');
                        if tx_err
                            .send(Ok(InstallDependenciesResponse {
                                stdout: vec![],
                                stderr: bytes,
                            }))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                }
            });

            // Stream stdout
            let stdout = child.stdout.take();
            let tx_out = tx.clone();
            let stdout_task = tokio::spawn(async move {
                if let Some(stdout) = stdout {
                    use tokio::io::AsyncBufReadExt;
                    let reader = tokio::io::BufReader::new(stdout);
                    let mut lines = reader.lines();
                    while let Ok(Some(line)) = lines.next_line().await {
                        let mut bytes = line.into_bytes();
                        bytes.push(b'\n');
                        if tx_out
                            .send(Ok(InstallDependenciesResponse {
                                stdout: bytes,
                                stderr: vec![],
                            }))
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                }
            });

            let _ = tokio::join!(stderr_task, stdout_task);

            match child.wait().await {
                Ok(status) if !status.success() => {
                    let code = status.code().unwrap_or(-1);
                    let msg = format!("cargo build exited with code {code}\n");
                    let _ = tx
                        .send(Ok(InstallDependenciesResponse {
                            stdout: vec![],
                            stderr: msg.into_bytes(),
                        }))
                        .await;
                }
                Err(e) => {
                    let msg = format!("error waiting for cargo build: {e}\n");
                    let _ = tx
                        .send(Ok(InstallDependenciesResponse {
                            stdout: vec![],
                            stderr: msg.into_bytes(),
                        }))
                        .await;
                }
                _ => {}
            }
        });

        Ok(Response::new(tokio_stream::wrappers::ReceiverStream::new(
            rx,
        )))
    }

    async fn runtime_options_prompts(
        &self,
        _request: Request<RuntimeOptionsRequest>,
    ) -> Result<Response<RuntimeOptionsResponse>, Status> {
        Ok(Response::new(RuntimeOptionsResponse { prompts: vec![] }))
    }

    async fn template(
        &self,
        request: Request<TemplateRequest>,
    ) -> Result<Response<TemplateResponse>, Status> {
        let req = request.into_inner();
        let project_name = &req.project_name;
        let work_dir = req
            .info
            .as_ref()
            .map(|i| i.program_directory.as_str())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| Status::invalid_argument("program_directory is required"))?;

        info!("Template: project={project_name}, dir={work_dir}");

        let dir = std::path::Path::new(work_dir);
        std::fs::create_dir_all(dir.join("src"))
            .map_err(|e| Status::internal(format!("failed to create src dir: {e}")))?;

        // Cargo.toml
        let cargo_toml = format!(
            r#"[package]
name = "{project_name}"
version = "0.1.0"
edition = "2021"

[dependencies]
pulumi-sdk = "0.1"
tokio = {{ version = "1", features = ["full"] }}
serde_json = "1"
"#
        );
        std::fs::write(dir.join("Cargo.toml"), cargo_toml)
            .map_err(|e| Status::internal(format!("failed to write Cargo.toml: {e}")))?;

        // src/main.rs
        let main_rs = r#"use pulumi_sdk::{run, Result};

fn main() -> Result<()> {
    run(|ctx| async move {
        // TODO: Create resources here
        ctx.export("output_key", serde_json::Value::String("value".into())).await;
        Ok(())
    })
}
"#;
        std::fs::write(dir.join("src").join("main.rs"), main_rs)
            .map_err(|e| Status::internal(format!("failed to write src/main.rs: {e}")))?;

        Ok(Response::new(TemplateResponse::default()))
    }

    async fn about(
        &self,
        _request: Request<AboutRequest>,
    ) -> Result<Response<AboutResponse>, Status> {
        Ok(Response::new(AboutResponse {
            executable: "pulumi-language-rust".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            metadata: Default::default(),
        }))
    }

    async fn get_program_dependencies(
        &self,
        _request: Request<GetProgramDependenciesRequest>,
    ) -> Result<Response<GetProgramDependenciesResponse>, Status> {
        Ok(Response::new(GetProgramDependenciesResponse {
            dependencies: vec![],
        }))
    }

    type RunPluginStream =
        tokio_stream::wrappers::ReceiverStream<Result<RunPluginResponse, Status>>;

    async fn run_plugin(
        &self,
        _request: Request<RunPluginRequest>,
    ) -> Result<Response<Self::RunPluginStream>, Status> {
        Err(Status::unimplemented("RunPlugin not supported"))
    }

    async fn generate_program(
        &self,
        _request: Request<GenerateProgramRequest>,
    ) -> Result<Response<GenerateProgramResponse>, Status> {
        Err(Status::unimplemented("GenerateProgram not supported"))
    }

    async fn generate_project(
        &self,
        _request: Request<GenerateProjectRequest>,
    ) -> Result<Response<GenerateProjectResponse>, Status> {
        Err(Status::unimplemented("GenerateProject not supported"))
    }

    async fn generate_package(
        &self,
        _request: Request<GeneratePackageRequest>,
    ) -> Result<Response<GeneratePackageResponse>, Status> {
        Err(Status::unimplemented("GeneratePackage not supported"))
    }

    async fn pack(&self, _request: Request<PackRequest>) -> Result<Response<PackResponse>, Status> {
        Err(Status::unimplemented("Pack not supported"))
    }

    async fn link(&self, _request: Request<LinkRequest>) -> Result<Response<LinkResponse>, Status> {
        Err(Status::unimplemented("Link not supported"))
    }

    async fn cancel(&self, _request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}

/// Resolve the binary to run for a Pulumi Rust program.
///
/// Strategy:
/// 1. If entry_point is an absolute path or points to an existing file, use it directly
/// 2. Look for a pre-built binary in target/release/ based on Cargo.toml package name
/// 3. If not found, run `cargo build --release` and try again
async fn resolve_binary(work_dir: &str, entry_point: &str) -> std::result::Result<String, String> {
    let work = std::path::Path::new(work_dir);

    // If entry_point is a real file path (not "."), use it directly
    if entry_point != "." {
        let ep = std::path::Path::new(entry_point);
        if ep.is_absolute() && ep.exists() {
            return Ok(entry_point.to_string());
        }
        let joined = work.join(entry_point);
        if joined.exists() {
            return Ok(joined.to_string_lossy().into_owned());
        }
    }

    // Discover binary name from Cargo.toml
    let bin_name = discover_bin_name(work)?;

    // Check if pre-built release binary exists
    let release_bin = work.join("target").join("release").join(&bin_name);
    if release_bin.exists() {
        return Ok(release_bin.to_string_lossy().into_owned());
    }

    // Also check debug binary
    let debug_bin = work.join("target").join("debug").join(&bin_name);
    if debug_bin.exists() {
        return Ok(debug_bin.to_string_lossy().into_owned());
    }

    // No binary found — build it
    info!("No pre-built binary found, running cargo build --release");
    let status = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .current_dir(work)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .map_err(|e| format!("failed to spawn cargo build: {e}"))?;

    if !status.success() {
        return Err(format!(
            "cargo build --release failed with code {}",
            status.code().unwrap_or(-1)
        ));
    }

    // Now the binary should exist
    if release_bin.exists() {
        return Ok(release_bin.to_string_lossy().into_owned());
    }

    Err(format!(
        "binary '{}' not found after build",
        release_bin.display()
    ))
}

/// Discover the binary name from Cargo.toml.
/// Checks [[bin]] targets first, then falls back to package.name.
fn discover_bin_name(work_dir: &std::path::Path) -> std::result::Result<String, String> {
    let cargo_path = work_dir.join("Cargo.toml");
    let contents = std::fs::read_to_string(&cargo_path)
        .map_err(|e| format!("cannot read {}: {e}", cargo_path.display()))?;

    let doc: toml::Table = contents
        .parse()
        .map_err(|e| format!("cannot parse Cargo.toml: {e}"))?;

    // Check [[bin]] targets
    if let Some(bins) = doc.get("bin").and_then(|b| b.as_array()) {
        if let Some(first) = bins.first() {
            if let Some(name) = first.get("name").and_then(|n| n.as_str()) {
                return Ok(name.to_string());
            }
        }
    }

    // Fall back to package.name (replace hyphens with underscores for binary name)
    if let Some(pkg) = doc.get("package").and_then(|p| p.as_table()) {
        if let Some(name) = pkg.get("name").and_then(|n| n.as_str()) {
            return Ok(name.replace('-', "_"));
        }
    }

    Err("cannot determine binary name from Cargo.toml".to_string())
}

/// Maps a Rust crate name to a Pulumi provider package name.
/// e.g., "pulumi-azure-native" -> "azure-native", "pulumi-random" -> "random"
/// Returns None for non-provider crates like "pulumi-sdk".
fn crate_to_provider(crate_name: &str) -> Option<String> {
    let stripped = crate_name.strip_prefix("pulumi-")?;
    // Skip SDK/tooling crates
    match stripped {
        "sdk" | "proto" | "codegen" | "codegen-cli" | "language-rust" => None,
        provider => Some(provider.to_string()),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let host = RustLanguageHost::new();

    // Bind to a random port
    let addr: SocketAddr = "127.0.0.1:0".parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let local_addr = listener.local_addr()?;

    // Print the port to stdout — the Pulumi CLI reads this to connect
    println!("{}", local_addr.port());

    info!("Language host listening on {local_addr}");

    let incoming = tokio_stream::wrappers::TcpListenerStream::new(listener);

    Server::builder()
        .add_service(LanguageRuntimeServer::new(host))
        .serve_with_incoming(incoming)
        .await?;

    Ok(())
}
