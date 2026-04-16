use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified route from a route table.
#[derive(Default)]
pub struct GetRouteArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the route.
    pub route_name: String,
    /// The name of the route table.
    pub route_table_name: String,
}

/// Result of the function invocation.
pub struct GetRouteResult {
    /// The destination CIDR to which the route applies.
    pub address_prefix: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// A value indicating whether this route overrides overlapping BGP routes regardless of LPM.
    pub has_bgp_override: bool,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The IP address packets should be forwarded to. Next hop values are only allowed in routes where the next hop type is VirtualAppliance.
    pub next_hop_ip_address: Option<String>,
    /// The type of Azure hop the packet should be sent to.
    pub next_hop_type: String,
    /// The provisioning state of the route resource.
    pub provisioning_state: String,
    /// The type of the resource.
    pub type_: Option<String>,
}

/// Gets the specified route from a route table.
pub async fn get_route(
    ctx: &Context,
    args: GetRouteArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetRouteResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routeName".to_string(), json!(args.route_name));
    invoke_args.insert("routeTableName".to_string(), json!(args.route_table_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220501:getRoute", invoke_args, &opts).await?;

    Ok(GetRouteResult {
        address_prefix: result.fields.get("addressPrefix").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        has_bgp_override: serde_json::from_value(result.fields.get("hasBgpOverride").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        next_hop_ip_address: result.fields.get("nextHopIpAddress").cloned().map(serde_json::from_value).transpose()?,
        next_hop_type: serde_json::from_value(result.fields.get("nextHopType").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: result.fields.get("type").cloned().map(serde_json::from_value).transpose()?,
    })
}
