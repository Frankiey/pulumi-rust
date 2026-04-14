use std::collections::HashMap;

use anyhow::{Context, Result};
use prost_types::Struct;
use tonic::transport::Channel;

use pulumi_proto::pulumirpc::{
    self, engine_client::EngineClient, resource_monitor_client::ResourceMonitorClient,
    RegisterResourceOutputsRequest, RegisterResourceRequest, SetRootResourceRequest,
};

use crate::serialize;

/// Response from registering a resource with the Pulumi engine.
#[derive(Debug)]
pub struct RegisterResourceResponse {
    pub urn: String,
    pub id: String,
    pub properties: HashMap<String, serialize::PropertyValue>,
}

/// The gRPC client layer connecting to the Pulumi engine.
pub struct GrpcEngine {
    monitor: ResourceMonitorClient<Channel>,
    engine: EngineClient<Channel>,
}

impl GrpcEngine {
    /// Connect to the ResourceMonitor and Engine services using addresses from env vars.
    pub async fn connect(monitor_addr: &str, engine_addr: &str) -> Result<Self> {
        let monitor_url = format!("http://{monitor_addr}");
        let engine_url = format!("http://{engine_addr}");

        let monitor = ResourceMonitorClient::connect(monitor_url)
            .await
            .context("failed to connect to ResourceMonitor")?;
        let engine = EngineClient::connect(engine_url)
            .await
            .context("failed to connect to Engine")?;

        Ok(Self { monitor, engine })
    }

    /// Register a resource with the Pulumi engine.
    #[allow(clippy::too_many_arguments)]
    pub async fn register_resource(
        &mut self,
        resource_type: &str,
        name: &str,
        custom: bool,
        inputs: Struct,
        parent: &str,
        dependencies: Vec<String>,
        property_deps: HashMap<String, Vec<String>>,
    ) -> Result<RegisterResourceResponse> {
        let prop_deps = property_deps
            .into_iter()
            .map(|(k, urns)| {
                (
                    k,
                    pulumirpc::register_resource_request::PropertyDependencies { urns },
                )
            })
            .collect();

        let req = RegisterResourceRequest {
            r#type: resource_type.to_string(),
            name: name.to_string(),
            parent: parent.to_string(),
            custom,
            object: Some(inputs),
            dependencies,
            property_dependencies: prop_deps,
            accept_secrets: true,
            accept_resources: true,
            ..Default::default()
        };

        let resp = self
            .monitor
            .register_resource(req)
            .await
            .context("RegisterResource RPC failed")?
            .into_inner();

        let properties = resp
            .object
            .as_ref()
            .map(serialize::deserialize_properties)
            .unwrap_or_default();

        Ok(RegisterResourceResponse {
            urn: resp.urn,
            id: resp.id,
            properties,
        })
    }

    /// Register the stack resource outputs.
    pub async fn register_resource_outputs(&mut self, urn: &str, outputs: Struct) -> Result<()> {
        let req = RegisterResourceOutputsRequest {
            urn: urn.to_string(),
            outputs: Some(outputs),
        };

        self.monitor
            .register_resource_outputs(req)
            .await
            .context("RegisterResourceOutputs RPC failed")?;

        Ok(())
    }

    /// Set the root resource (stack) URN.
    pub async fn set_root_resource(&mut self, urn: &str) -> Result<()> {
        let req = SetRootResourceRequest {
            urn: urn.to_string(),
        };

        self.engine
            .set_root_resource(req)
            .await
            .context("SetRootResource RPC failed")?;

        Ok(())
    }

    /// Log a message to the Pulumi engine.
    pub async fn log(&mut self, severity: i32, message: &str, urn: &str) -> Result<()> {
        let req = pulumirpc::LogRequest {
            severity,
            message: message.to_string(),
            urn: urn.to_string(),
            stream_id: 0,
            ephemeral: false,
        };

        self.engine.log(req).await.context("Log RPC failed")?;

        Ok(())
    }
}
