use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified authorization from the specified express route circuit.
#[derive(Default)]
pub struct GetExpressRouteCircuitAuthorizationArgs {
    /// The name of the authorization.
    pub authorization_name: String,
    /// The name of the express route circuit.
    pub circuit_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRouteCircuitAuthorizationResult {
    /// The authorization key.
    pub authorization_key: Option<String>,
    /// The authorization use status.
    pub authorization_use_status: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Gets name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// Type of the resource.
    pub type_: String,
}

/// Gets the specified authorization from the specified express route circuit.
pub async fn get_express_route_circuit_authorization(
    ctx: &Context,
    args: GetExpressRouteCircuitAuthorizationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRouteCircuitAuthorizationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("authorizationName".to_string(), json!(args.authorization_name));
    invoke_args.insert("circuitName".to_string(), json!(args.circuit_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20190601:getExpressRouteCircuitAuthorization", invoke_args, &opts).await?;

    Ok(GetExpressRouteCircuitAuthorizationResult {
        authorization_key: result.fields.get("authorizationKey").cloned().map(serde_json::from_value).transpose()?,
        authorization_use_status: result.fields.get("authorizationUseStatus").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
