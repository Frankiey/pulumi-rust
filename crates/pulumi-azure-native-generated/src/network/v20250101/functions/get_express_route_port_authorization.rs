use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified authorization from the specified express route port.
#[derive(Default)]
pub struct GetExpressRoutePortAuthorizationArgs {
    /// The name of the authorization.
    pub authorization_name: String,
    /// The name of the express route port.
    pub express_route_port_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRoutePortAuthorizationResult {
    /// The authorization key.
    pub authorization_key: String,
    /// The authorization use status.
    pub authorization_use_status: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The reference to the ExpressRoute circuit resource using the authorization.
    pub circuit_resource_uri: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the authorization resource.
    pub provisioning_state: String,
    /// Type of the resource.
    pub type_: String,
}

/// Gets the specified authorization from the specified express route port.
pub async fn get_express_route_port_authorization(
    ctx: &Context,
    args: GetExpressRoutePortAuthorizationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRoutePortAuthorizationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("authorizationName".to_string(), json!(args.authorization_name));
    invoke_args.insert("expressRoutePortName".to_string(), json!(args.express_route_port_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250101:getExpressRoutePortAuthorization", invoke_args, &opts).await?;

    Ok(GetExpressRoutePortAuthorizationResult {
        authorization_key: serde_json::from_value(result.fields.get("authorizationKey").cloned().unwrap_or_default())?
            ,
        authorization_use_status: serde_json::from_value(result.fields.get("authorizationUseStatus").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        circuit_resource_uri: serde_json::from_value(result.fields.get("circuitResourceUri").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
