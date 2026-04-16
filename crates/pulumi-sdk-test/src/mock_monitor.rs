use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use prost_types::Struct;
use tokio::sync::Mutex;
use tonic::{Request, Response, Status};

use pulumi_proto::pulumirpc;
use pulumi_sdk::Context;

/// User-provided mock behavior for resource creation and function calls.
pub trait Mocks: Send + Sync + 'static {
    /// Called when a resource is registered.
    ///
    /// Return a tuple of (id, output_properties) for the resource.
    /// `resource_type` is the Pulumi type token (e.g., "aws:s3/bucket:Bucket").
    /// `name` is the logical name given by the user.
    /// `inputs` is the input properties as a JSON map.
    fn new_resource(
        &self,
        resource_type: &str,
        name: &str,
        inputs: &serde_json::Value,
    ) -> Result<(String, serde_json::Value), String>;

    /// Called when a provider function is invoked.
    ///
    /// Return the function result as a JSON object.
    fn call(&self, token: &str, args: &serde_json::Value) -> Result<serde_json::Value, String> {
        let _ = (token, args);
        Ok(serde_json::json!({}))
    }
}

/// A resource that was registered during the test run.
#[derive(Debug, Clone)]
pub struct MockResource {
    /// The Pulumi type token.
    pub resource_type: String,
    /// The logical resource name.
    pub name: String,
    /// The input properties.
    pub inputs: serde_json::Value,
    /// The URN assigned to this resource.
    pub urn: String,
    /// The provider-assigned ID.
    pub id: String,
    /// The output properties returned by the mock.
    pub outputs: serde_json::Value,
}

/// Results from running a test with mocks.
#[derive(Debug)]
pub struct TestResult {
    /// All resources that were registered.
    pub resources: Vec<MockResource>,
    /// Stack exports (key → JSON value).
    pub exports: HashMap<String, serde_json::Value>,
}

// Internal state shared between the mock server and the test runner.
struct MockState {
    mocks: Box<dyn Mocks>,
    resources: Vec<MockResource>,
    root_urn: String,
    exports: HashMap<String, serde_json::Value>,
    project: String,
    stack: String,
    resource_counter: u64,
}

struct MockMonitorService {
    state: Arc<Mutex<MockState>>,
}

struct MockEngineService {
    state: Arc<Mutex<MockState>>,
}

fn struct_to_json(s: &Struct) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for (key, value) in &s.fields {
        map.insert(key.clone(), prost_value_to_json(value));
    }
    serde_json::Value::Object(map)
}

fn prost_value_to_json(v: &prost_types::Value) -> serde_json::Value {
    use prost_types::value::Kind;
    match &v.kind {
        Some(Kind::NullValue(_)) => serde_json::Value::Null,
        Some(Kind::NumberValue(n)) => serde_json::json!(n),
        Some(Kind::StringValue(s)) => serde_json::json!(s),
        Some(Kind::BoolValue(b)) => serde_json::json!(b),
        Some(Kind::StructValue(s)) => struct_to_json(s),
        Some(Kind::ListValue(l)) => {
            serde_json::Value::Array(l.values.iter().map(prost_value_to_json).collect())
        }
        None => serde_json::Value::Null,
    }
}

fn json_to_struct(val: &serde_json::Value) -> Struct {
    let mut fields = std::collections::BTreeMap::new();
    if let serde_json::Value::Object(map) = val {
        for (k, v) in map {
            fields.insert(k.clone(), json_to_prost_value(v));
        }
    }
    Struct { fields }
}

fn json_to_prost_value(v: &serde_json::Value) -> prost_types::Value {
    use prost_types::value::Kind;
    let kind = match v {
        serde_json::Value::Null => Kind::NullValue(0),
        serde_json::Value::Bool(b) => Kind::BoolValue(*b),
        serde_json::Value::Number(n) => Kind::NumberValue(n.as_f64().unwrap_or(0.0)),
        serde_json::Value::String(s) => Kind::StringValue(s.clone()),
        serde_json::Value::Array(arr) => Kind::ListValue(prost_types::ListValue {
            values: arr.iter().map(json_to_prost_value).collect(),
        }),
        serde_json::Value::Object(_) => Kind::StructValue(json_to_struct(v)),
    };
    prost_types::Value { kind: Some(kind) }
}

#[tonic::async_trait]
impl pulumirpc::resource_monitor_server::ResourceMonitor for MockMonitorService {
    async fn supports_feature(
        &self,
        _request: Request<pulumirpc::SupportsFeatureRequest>,
    ) -> Result<Response<pulumirpc::SupportsFeatureResponse>, Status> {
        Ok(Response::new(pulumirpc::SupportsFeatureResponse {
            has_support: true,
        }))
    }

    async fn register_resource(
        &self,
        request: Request<pulumirpc::RegisterResourceRequest>,
    ) -> Result<Response<pulumirpc::RegisterResourceResponse>, Status> {
        let req = request.into_inner();
        let mut state = self.state.lock().await;

        // Stack registration
        if req.r#type == "pulumi:pulumi:Stack" {
            let urn = format!(
                "urn:pulumi:{}::{}::pulumi:pulumi:Stack::{}",
                state.stack, state.project, req.name
            );
            return Ok(Response::new(pulumirpc::RegisterResourceResponse {
                urn,
                object: Some(Struct::default()),
                ..Default::default()
            }));
        }

        state.resource_counter += 1;
        let inputs_json = req
            .object
            .as_ref()
            .map(struct_to_json)
            .unwrap_or_else(|| serde_json::json!({}));

        let urn = format!(
            "urn:pulumi:{}::{}::{}::{}",
            state.stack, state.project, req.r#type, req.name
        );

        let (id, outputs) = state
            .mocks
            .new_resource(&req.r#type, &req.name, &inputs_json)
            .map_err(|e| Status::internal(e))?;

        let resource = MockResource {
            resource_type: req.r#type.clone(),
            name: req.name.clone(),
            inputs: inputs_json,
            urn: urn.clone(),
            id: id.clone(),
            outputs: outputs.clone(),
        };
        state.resources.push(resource);

        let output_struct = json_to_struct(&outputs);

        Ok(Response::new(pulumirpc::RegisterResourceResponse {
            urn,
            id,
            object: Some(output_struct),
            ..Default::default()
        }))
    }

    async fn register_resource_outputs(
        &self,
        request: Request<pulumirpc::RegisterResourceOutputsRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();

        // Capture stack exports
        if let Some(outputs) = &req.outputs {
            let json = struct_to_json(outputs);
            if let serde_json::Value::Object(map) = json {
                let mut state = self.state.lock().await;
                for (k, v) in map {
                    state.exports.insert(k, v);
                }
            }
        }

        Ok(Response::new(()))
    }

    async fn invoke(
        &self,
        request: Request<pulumirpc::ResourceInvokeRequest>,
    ) -> Result<Response<pulumirpc::InvokeResponse>, Status> {
        let req = request.into_inner();
        let state = self.state.lock().await;

        let args_json = req
            .args
            .as_ref()
            .map(struct_to_json)
            .unwrap_or_else(|| serde_json::json!({}));

        let result = state
            .mocks
            .call(&req.tok, &args_json)
            .map_err(|e| Status::internal(e))?;

        let result_struct = json_to_struct(&result);

        Ok(Response::new(pulumirpc::InvokeResponse {
            r#return: Some(result_struct),
            failures: vec![],
        }))
    }

    async fn call(
        &self,
        _request: Request<pulumirpc::ResourceCallRequest>,
    ) -> Result<Response<pulumirpc::CallResponse>, Status> {
        Ok(Response::new(pulumirpc::CallResponse::default()))
    }

    async fn read_resource(
        &self,
        _request: Request<pulumirpc::ReadResourceRequest>,
    ) -> Result<Response<pulumirpc::ReadResourceResponse>, Status> {
        Err(Status::unimplemented("ReadResource not supported in mocks"))
    }

    async fn register_stack_transform(
        &self,
        _request: Request<pulumirpc::Callback>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn register_stack_invoke_transform(
        &self,
        _request: Request<pulumirpc::Callback>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn register_resource_hook(
        &self,
        _request: Request<pulumirpc::RegisterResourceHookRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn register_error_hook(
        &self,
        _request: Request<pulumirpc::RegisterErrorHookRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn register_package(
        &self,
        _request: Request<pulumirpc::RegisterPackageRequest>,
    ) -> Result<Response<pulumirpc::RegisterPackageResponse>, Status> {
        Ok(Response::new(pulumirpc::RegisterPackageResponse {
            r#ref: "mock-package-ref".to_string(),
        }))
    }

    async fn signal_and_wait_for_shutdown(
        &self,
        _request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}

#[tonic::async_trait]
impl pulumirpc::engine_server::Engine for MockEngineService {
    async fn log(&self, _request: Request<pulumirpc::LogRequest>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn get_root_resource(
        &self,
        _request: Request<pulumirpc::GetRootResourceRequest>,
    ) -> Result<Response<pulumirpc::GetRootResourceResponse>, Status> {
        let state = self.state.lock().await;
        Ok(Response::new(pulumirpc::GetRootResourceResponse {
            urn: state.root_urn.clone(),
        }))
    }

    async fn set_root_resource(
        &self,
        request: Request<pulumirpc::SetRootResourceRequest>,
    ) -> Result<Response<pulumirpc::SetRootResourceResponse>, Status> {
        let req = request.into_inner();
        self.state.lock().await.root_urn = req.urn;
        Ok(Response::new(pulumirpc::SetRootResourceResponse {}))
    }

    async fn start_debugging(
        &self,
        _request: Request<pulumirpc::StartDebuggingRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn require_pulumi_version(
        &self,
        _request: Request<pulumirpc::RequirePulumiVersionRequest>,
    ) -> Result<Response<pulumirpc::RequirePulumiVersionResponse>, Status> {
        Ok(Response::new(pulumirpc::RequirePulumiVersionResponse {}))
    }
}

/// Run a Pulumi program with mock resources for unit testing.
///
/// This starts a mock gRPC server, sets the necessary env vars, and runs
/// the program through `pulumi_sdk::run_async`. Returns all registered
/// resources and stack exports.
///
/// # Example
/// ```ignore
/// use pulumi_sdk_test::{test_with_mocks, Mocks};
///
/// struct MyMocks;
/// impl Mocks for MyMocks {
///     fn new_resource(&self, resource_type: &str, name: &str, inputs: &serde_json::Value)
///         -> Result<(String, serde_json::Value), String>
///     {
///         Ok(("test-id".into(), inputs.clone()))
///     }
/// }
///
/// #[tokio::test]
/// async fn test_my_infra() {
///     let result = test_with_mocks(MyMocks, |ctx| async move {
///         // register resources with ctx...
///         Ok(())
///     }).await.unwrap();
///     assert!(!result.resources.is_empty());
/// }
/// ```
pub async fn test_with_mocks<M, F, Fut>(
    mocks: M,
    project: &str,
    stack: &str,
    f: F,
) -> pulumi_sdk::Result<TestResult>
where
    M: Mocks,
    F: FnOnce(Context) -> Fut + Send + 'static,
    Fut: std::future::Future<Output = pulumi_sdk::Result<()>> + Send + 'static,
{
    let state = Arc::new(Mutex::new(MockState {
        mocks: Box::new(mocks),
        resources: Vec::new(),
        root_urn: String::new(),
        exports: HashMap::new(),
        project: project.to_string(),
        stack: stack.to_string(),
        resource_counter: 0,
    }));

    // Start monitor server
    let monitor_svc = MockMonitorService {
        state: Arc::clone(&state),
    };
    let monitor_addr: SocketAddr = "127.0.0.1:0".parse().expect("valid addr");
    let monitor_listener = tokio::net::TcpListener::bind(monitor_addr)
        .await
        .map_err(|e| pulumi_sdk::PulumiError::Custom(format!("failed to bind monitor: {e}")))?;
    let monitor_port = monitor_listener
        .local_addr()
        .map_err(|e| pulumi_sdk::PulumiError::Custom(format!("failed to get monitor addr: {e}")))?
        .port();

    // Start engine server
    let engine_svc = MockEngineService {
        state: Arc::clone(&state),
    };
    let engine_addr: SocketAddr = "127.0.0.1:0".parse().expect("valid addr");
    let engine_listener = tokio::net::TcpListener::bind(engine_addr)
        .await
        .map_err(|e| pulumi_sdk::PulumiError::Custom(format!("failed to bind engine: {e}")))?;
    let engine_port = engine_listener
        .local_addr()
        .map_err(|e| pulumi_sdk::PulumiError::Custom(format!("failed to get engine addr: {e}")))?
        .port();

    let monitor_incoming = tokio_stream::wrappers::TcpListenerStream::new(monitor_listener);
    let engine_incoming = tokio_stream::wrappers::TcpListenerStream::new(engine_listener);

    // Spawn both servers
    let _monitor_handle = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(
                pulumirpc::resource_monitor_server::ResourceMonitorServer::new(monitor_svc),
            )
            .serve_with_incoming(monitor_incoming)
            .await
    });

    let _engine_handle = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(pulumirpc::engine_server::EngineServer::new(engine_svc))
            .serve_with_incoming(engine_incoming)
            .await
    });

    // Give servers a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    // Set env vars for the SDK
    let monitor_env = format!("127.0.0.1:{monitor_port}");
    let engine_env = format!("127.0.0.1:{engine_port}");

    std::env::set_var("PULUMI_MONITOR", &monitor_env);
    std::env::set_var("PULUMI_ENGINE", &engine_env);
    std::env::set_var("PULUMI_PROJECT", project);
    std::env::set_var("PULUMI_STACK", stack);
    std::env::set_var("PULUMI_DRY_RUN", "true");

    // Run the program
    pulumi_sdk::run_async(f).await?;

    // Collect results
    let final_state = state.lock().await;

    Ok(TestResult {
        resources: final_state.resources.clone(),
        exports: final_state.exports.clone(),
    })
}
