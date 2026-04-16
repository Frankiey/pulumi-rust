use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the backend health for given combination of backend pool and http setting of the specified application gateway in a resource group.
#[derive(Default)]
pub struct GetApplicationGatewayBackendHealthOnDemandArgs {
    /// The name of the application gateway.
    pub application_gateway_name: String,
    /// Reference to backend pool of application gateway to which probe request will be sent.
    pub backend_address_pool: Option<network::v20210301::SubResource>,
    /// Reference to backend http setting of application gateway to be used for test probe.
    pub backend_http_settings: Option<network::v20210301::SubResource>,
    /// Expands BackendAddressPool and BackendHttpSettings referenced in backend health.
    pub expand: Option<String>,
    /// Host name to send the probe to.
    pub host: Option<String>,
    /// Criterion for classifying a healthy probe response.
    pub match_: Option<network::v20210301::ApplicationGatewayProbeHealthResponseMatch>,
    /// Relative path of probe. Valid path starts from '/'. Probe is sent to <Protocol>://<host>:<port><path>.
    pub path: Option<String>,
    /// Whether the host header should be picked from the backend http settings. Default value is false.
    pub pick_host_name_from_backend_http_settings: Option<bool>,
    /// The protocol used for the probe.
    pub protocol: Option<serde_json::Value>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The probe timeout in seconds. Probe marked as failed if valid response is not received with this timeout period. Acceptable values are from 1 second to 86400 seconds.
    pub timeout: Option<i64>,
}

/// Result of the function invocation.
pub struct GetApplicationGatewayBackendHealthOnDemandResult {
    /// Reference to an ApplicationGatewayBackendAddressPool resource.
    pub backend_address_pool: Option<network::v20210301::ApplicationGatewayBackendAddressPoolResponse>,
    /// Application gateway BackendHealthHttp settings.
    pub backend_health_http_settings: Option<network::v20210301::ApplicationGatewayBackendHealthHttpSettingsResponse>,
}

/// Gets the backend health for given combination of backend pool and http setting of the specified application gateway in a resource group.
pub async fn get_application_gateway_backend_health_on_demand(
    ctx: &Context,
    args: GetApplicationGatewayBackendHealthOnDemandArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetApplicationGatewayBackendHealthOnDemandResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("applicationGatewayName".to_string(), json!(args.application_gateway_name));
    if let Some(v) = args.backend_address_pool {
        invoke_args.insert("backendAddressPool".to_string(), json!(v));
    }
    if let Some(v) = args.backend_http_settings {
        invoke_args.insert("backendHttpSettings".to_string(), json!(v));
    }
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    if let Some(v) = args.host {
        invoke_args.insert("host".to_string(), json!(v));
    }
    if let Some(v) = args.match_ {
        invoke_args.insert("match".to_string(), json!(v));
    }
    if let Some(v) = args.path {
        invoke_args.insert("path".to_string(), json!(v));
    }
    if let Some(v) = args.pick_host_name_from_backend_http_settings {
        invoke_args.insert("pickHostNameFromBackendHttpSettings".to_string(), json!(v));
    }
    if let Some(v) = args.protocol {
        invoke_args.insert("protocol".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.timeout {
        invoke_args.insert("timeout".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20210301:getApplicationGatewayBackendHealthOnDemand", invoke_args, &opts).await?;

    Ok(GetApplicationGatewayBackendHealthOnDemandResult {
        backend_address_pool: result.fields.get("backendAddressPool").cloned().map(serde_json::from_value).transpose()?,
        backend_health_http_settings: result.fields.get("backendHealthHttpSettings").cloned().map(serde_json::from_value).transpose()?,
    })
}
