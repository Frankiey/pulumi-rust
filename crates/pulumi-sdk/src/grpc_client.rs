use std::collections::HashMap;

use prost_types::Struct;
use tonic::transport::Channel;

use pulumi_proto::pulumirpc::{
    self, engine_client::EngineClient, resource_monitor_client::ResourceMonitorClient,
    RegisterPackageRequest, RegisterResourceOutputsRequest, RegisterResourceRequest,
    ResourceInvokeRequest, SetRootResourceRequest, SupportsFeatureRequest,
};

use crate::error::{PulumiError, Result};
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

        let monitor = ResourceMonitorClient::connect(monitor_url).await?;
        let engine = EngineClient::connect(engine_url).await?;

        Ok(Self { monitor, engine })
    }

    /// Query whether the resource monitor supports a specific feature.
    pub async fn supports_feature(&mut self, feature: &str) -> Result<bool> {
        let req = SupportsFeatureRequest {
            id: feature.to_string(),
        };

        let resp = self
            .monitor
            .supports_feature(req)
            .await
            .map_err(|e| PulumiError::Custom(format!("SupportsFeature RPC failed: {e}")))?
            .into_inner();

        Ok(resp.has_support)
    }

    /// Register a provider package and get a package reference UUID.
    ///
    /// Generated provider code calls this once at startup and uses the
    /// returned `packageRef` in subsequent `RegisterResource` calls.
    pub async fn register_package(
        &mut self,
        name: &str,
        version: &str,
        download_url: &str,
        parameterization: Option<crate::Parameterization>,
    ) -> Result<String> {
        let proto_param = parameterization.map(|p| pulumirpc::Parameterization {
            name: p.name,
            version: p.version,
            value: p.value,
        });

        let req = RegisterPackageRequest {
            name: name.to_string(),
            version: version.to_string(),
            download_url: download_url.to_string(),
            checksums: HashMap::new(),
            parameterization: proto_param,
        };

        let resp = self
            .monitor
            .register_package(req)
            .await
            .map_err(|e| PulumiError::Custom(format!("RegisterPackage RPC failed: {e}")))?
            .into_inner();

        Ok(resp.r#ref)
    }

    /// Register a resource with the Pulumi engine using a pre-built request.
    pub async fn register_resource(
        &mut self,
        req: RegisterResourceRequest,
    ) -> Result<RegisterResourceResponse> {
        let resp = self
            .monitor
            .register_resource(req)
            .await
            .map_err(|e| PulumiError::ResourceRegistration(e.to_string()))?
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

    /// Invoke a provider function via the ResourceMonitor.Invoke RPC.
    pub async fn invoke(
        &mut self,
        req: ResourceInvokeRequest,
    ) -> Result<HashMap<String, serialize::PropertyValue>> {
        let resp = self
            .monitor
            .invoke(req)
            .await
            .map_err(|e| PulumiError::InvokeFailed {
                token: String::new(),
                message: e.to_string(),
            })?
            .into_inner();

        // Check for failures
        if !resp.failures.is_empty() {
            let msgs: Vec<String> = resp
                .failures
                .iter()
                .map(|f| format!("{}: {}", f.property, f.reason))
                .collect();
            return Err(PulumiError::InvokeFailed {
                token: String::new(),
                message: msgs.join("; "),
            });
        }

        let properties = resp
            .r#return
            .as_ref()
            .map(serialize::deserialize_properties)
            .unwrap_or_default();

        Ok(properties)
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
            .map_err(|e| PulumiError::ResourceRegistration(e.to_string()))?;

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
            .map_err(|e| PulumiError::Custom(format!("SetRootResource RPC failed: {e}")))?;

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

        self.engine
            .log(req)
            .await
            .map_err(|e| PulumiError::Custom(format!("Log RPC failed: {e}")))?;

        Ok(())
    }
}
