use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified private endpoint connection on application gateway.
#[derive(Default)]
pub struct GetApplicationGatewayPrivateEndpointConnectionArgs {
    /// The name of the application gateway.
    pub application_gateway_name: String,
    /// The name of the application gateway private endpoint connection.
    pub connection_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetApplicationGatewayPrivateEndpointConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The consumer link id.
    pub link_identifier: String,
    /// Name of the private endpoint connection on an application gateway.
    pub name: Option<String>,
    /// The resource of private end point.
    pub private_endpoint: network::v20220701::PrivateEndpointResponse,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: Option<network::v20220701::PrivateLinkServiceConnectionStateResponse>,
    /// The provisioning state of the application gateway private endpoint connection resource.
    pub provisioning_state: String,
    /// Type of the resource.
    pub type_: String,
}

/// Gets the specified private endpoint connection on application gateway.
pub async fn get_application_gateway_private_endpoint_connection(
    ctx: &Context,
    args: GetApplicationGatewayPrivateEndpointConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetApplicationGatewayPrivateEndpointConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("applicationGatewayName".to_string(), json!(args.application_gateway_name));
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220701:getApplicationGatewayPrivateEndpointConnection", invoke_args, &opts).await?;

    Ok(GetApplicationGatewayPrivateEndpointConnectionResult {
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
