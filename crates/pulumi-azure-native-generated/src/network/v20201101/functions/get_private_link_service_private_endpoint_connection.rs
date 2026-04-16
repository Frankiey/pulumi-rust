use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get the specific private end point connection by specific private link service in the resource group.
#[derive(Default)]
pub struct GetPrivateLinkServicePrivateEndpointConnectionArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the private end point connection.
    pub pe_connection_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the private link service.
    pub service_name: String,
}

/// Result of the function invocation.
pub struct GetPrivateLinkServicePrivateEndpointConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The consumer link id.
    pub link_identifier: String,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The resource of private end point.
    pub private_endpoint: network::v20201101::PrivateEndpointResponse,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Option<network::v20201101::PrivateLinkServiceConnectionStateResponse>,
    /// The provisioning state of the private endpoint connection resource.
    pub provisioning_state: String,
    /// The resource type.
    pub type_: String,
}

/// Get the specific private end point connection by specific private link service in the resource group.
pub async fn get_private_link_service_private_endpoint_connection(
    ctx: &Context,
    args: GetPrivateLinkServicePrivateEndpointConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPrivateLinkServicePrivateEndpointConnectionResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("peConnectionName".to_string(), json!(args.pe_connection_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("serviceName".to_string(), json!(args.service_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20201101:getPrivateLinkServicePrivateEndpointConnection", invoke_args, &opts).await?;

    Ok(GetPrivateLinkServicePrivateEndpointConnectionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        link_identifier: serde_json::from_value(result.fields.get("linkIdentifier").cloned().unwrap_or_default())?
            ,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        private_endpoint: serde_json::from_value(result.fields.get("privateEndpoint").cloned().unwrap_or_default())?
            ,
        private_link_service_connection_state: result.fields.get("privateLinkServiceConnectionState").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
