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
        _request: Request<GetRequiredPackagesRequest>,
    ) -> Result<Response<GetRequiredPackagesResponse>, Status> {
        Ok(Response::new(GetRequiredPackagesResponse {
            packages: vec![],
        }))
    }

    async fn run(&self, request: Request<RunRequest>) -> Result<Response<RunResponse>, Status> {
        let req = request.into_inner();

        // Determine the program binary to execute.
        // Prefer ProgramInfo.entry_point (new API) over deprecated req.program.
        #[allow(deprecated)]
        let (work_dir, program) = if let Some(info) = &req.info {
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

        info!("Run: program={program}, pwd={work_dir}");

        let monitor_address = &req.monitor_address;
        let engine_address = self.engine_address.lock().await.clone();
        let project = &req.project;
        let stack = &req.stack;

        let mut cmd = Command::new(&program);
        cmd.current_dir(&work_dir)
            .env("PULUMI_MONITOR", monitor_address)
            .env("PULUMI_ENGINE", &engine_address)
            .env("PULUMI_PROJECT", project)
            .env("PULUMI_STACK", stack)
            .env("PULUMI_PARALLEL", req.parallel.to_string())
            .env("PULUMI_DRY_RUN", req.dry_run.to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit());

        // Pass config as env vars
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
        _request: Request<InstallDependenciesRequest>,
    ) -> Result<Response<Self::InstallDependenciesStream>, Status> {
        let (_tx, rx) = tokio::sync::mpsc::channel(1);
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
        _request: Request<TemplateRequest>,
    ) -> Result<Response<TemplateResponse>, Status> {
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
