use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a RouteMap.
#[derive(Default)]
pub struct GetRouteMapArgs {
    /// The resource group name of the RouteMap's resource group.
    pub resource_group_name: String,
    /// The name of the RouteMap.
    pub route_map_name: String,
    /// The name of the VirtualHub containing the RouteMap.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetRouteMapResult {
    /// List of connections which have this RoutMap associated for inbound traffic.
    pub associated_inbound_connections: Option<Vec<String>>,
    /// List of connections which have this RoutMap associated for outbound traffic.
    pub associated_outbound_connections: Option<Vec<String>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: String,
    /// The provisioning state of the RouteMap resource.
    pub provisioning_state: String,
    /// List of RouteMap rules to be applied.
    pub rules: Option<Vec<network::v20240101::RouteMapRuleResponse>>,
    /// Resource type.
    pub type_: String,
}

/// Retrieves the details of a RouteMap.
pub async fn get_route_map(
    ctx: &Context,
    args: GetRouteMapArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetRouteMapResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routeMapName".to_string(), json!(args.route_map_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240101:getRouteMap", invoke_args, &opts).await?;

    Ok(GetRouteMapResult {
        associated_inbound_connections: result.fields.get("associatedInboundConnections").cloned().map(serde_json::from_value).transpose()?,
        associated_outbound_connections: result.fields.get("associatedOutboundConnections").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        rules: result.fields.get("rules").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
