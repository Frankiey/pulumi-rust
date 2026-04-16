use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;

/// Result from `pulumi up` or `pulumi preview`.
#[derive(Debug, Clone)]
pub struct StackOutput {
    /// Stack outputs (from `pulumi stack output --json`).
    pub outputs: HashMap<String, serde_json::Value>,
    /// The raw stdout from the command.
    pub stdout: String,
    /// The raw stderr from the command.
    pub stderr: String,
}

/// Result from `pulumi destroy`.
#[derive(Debug, Clone)]
pub struct DestroyResult {
    pub stdout: String,
    pub stderr: String,
}

/// A test harness for running real `pulumi up` / `preview` / `destroy` cycles.
///
/// Tests using this harness should be marked `#[ignore]` so they only run when
/// explicitly invoked with `cargo test -- --ignored`.
///
/// # Example
///
/// ```rust,ignore
/// #[tokio::test]
/// #[ignore] // requires pulumi CLI + credentials
/// async fn test_stack_lifecycle() {
///     let stack = PulumiTestStack::new("examples/my-project", "test-stack").await.unwrap();
///     let up = stack.up().await.unwrap();
///     assert!(up.outputs.contains_key("url"));
///     stack.destroy().await.unwrap();
/// }
/// ```
pub struct PulumiTestStack {
    project_dir: PathBuf,
    stack_name: String,
    env: HashMap<String, String>,
}

impl PulumiTestStack {
    /// Create a new test stack. Initializes the stack via `pulumi stack init --non-interactive`.
    /// `project_dir` is the path to the Pulumi project (containing `Pulumi.yaml`).
    pub async fn new(
        project_dir: impl AsRef<Path>,
        stack_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let project_dir = std::fs::canonicalize(project_dir.as_ref())?;
        let stack = Self {
            project_dir: project_dir.clone(),
            stack_name: stack_name.to_string(),
            env: HashMap::new(),
        };

        // Try to select existing stack first, create if not found.
        let select = stack
            .run_pulumi(&["stack", "select", stack_name, "--non-interactive"])
            .await;
        if select.is_err() {
            stack
                .run_pulumi(&["stack", "init", stack_name, "--non-interactive"])
                .await?;
        }

        Ok(stack)
    }

    /// Set an environment variable for all pulumi commands.
    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env.insert(key.to_string(), value.to_string());
        self
    }

    /// Set a Pulumi config value.
    pub async fn set_config(
        &self,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.run_pulumi(&["config", "set", key, value, "--non-interactive"])
            .await?;
        Ok(())
    }

    /// Set a secret Pulumi config value.
    pub async fn set_config_secret(
        &self,
        key: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.run_pulumi(&["config", "set", "--secret", key, value, "--non-interactive"])
            .await?;
        Ok(())
    }

    /// Run `pulumi preview` and return the output.
    pub async fn preview(&self) -> Result<StackOutput, Box<dyn std::error::Error + Send + Sync>> {
        let (stdout, stderr) = self.run_pulumi(&["preview", "--non-interactive"]).await?;
        Ok(StackOutput {
            outputs: HashMap::new(), // preview doesn't produce outputs
            stdout,
            stderr,
        })
    }

    /// Run `pulumi up` and return the stack outputs.
    pub async fn up(&self) -> Result<StackOutput, Box<dyn std::error::Error + Send + Sync>> {
        let (stdout, stderr) = self
            .run_pulumi(&["up", "--yes", "--non-interactive"])
            .await?;

        let outputs = self.get_outputs().await.unwrap_or_default();

        Ok(StackOutput {
            outputs,
            stdout,
            stderr,
        })
    }

    /// Run `pulumi destroy` and clean up.
    pub async fn destroy(&self) -> Result<DestroyResult, Box<dyn std::error::Error + Send + Sync>> {
        let (stdout, stderr) = self
            .run_pulumi(&["destroy", "--yes", "--non-interactive"])
            .await?;
        Ok(DestroyResult { stdout, stderr })
    }

    /// Remove the stack completely (destroy + rm).
    pub async fn cleanup(self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Destroy first (ignore errors — stack might already be empty).
        let _ = self
            .run_pulumi(&["destroy", "--yes", "--non-interactive"])
            .await;
        self.run_pulumi(&["stack", "rm", "--yes", "--non-interactive"])
            .await?;
        Ok(())
    }

    /// Fetch stack outputs via `pulumi stack output --json`.
    async fn get_outputs(
        &self,
    ) -> Result<HashMap<String, serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
        let (stdout, _) = self
            .run_pulumi(&["stack", "output", "--json", "--non-interactive"])
            .await?;
        let outputs: HashMap<String, serde_json::Value> = serde_json::from_str(&stdout)?;
        Ok(outputs)
    }

    /// Run a pulumi CLI command and return (stdout, stderr).
    async fn run_pulumi(
        &self,
        args: &[&str],
    ) -> Result<(String, String), Box<dyn std::error::Error + Send + Sync>> {
        let mut cmd = tokio::process::Command::new("pulumi");
        cmd.args(args)
            .current_dir(&self.project_dir)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        for (k, v) in &self.env {
            cmd.env(k, v);
        }

        let output = cmd.spawn()?.wait_with_output().await?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(format!(
                "pulumi {} failed (exit {}):\nstdout: {}\nstderr: {}",
                args.join(" "),
                output.status.code().unwrap_or(-1),
                stdout,
                stderr,
            )
            .into());
        }

        Ok((stdout, stderr))
    }
}
