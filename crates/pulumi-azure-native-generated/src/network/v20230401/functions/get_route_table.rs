use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified route table.
#[derive(Default)]
pub struct GetRouteTableArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the route table.
    pub route_table_name: String,
}

/// Result of the function invocation.
pub struct GetRouteTableResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Whether to disable the routes learned by BGP on that route table. True means disable.
    pub disable_bgp_route_propagation: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the route table resource.
    pub provisioning_state: String,
    /// The resource GUID property of the route table.
    pub resource_guid: String,
    /// Collection of routes contained within a route table.
    pub routes: Option<Vec<network::v20230401::RouteResponse>>,
    /// A collection of references to subnets.
    pub subnets: Vec<network::v20230401::SubnetResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified route table.
pub async fn get_route_table(
    ctx: &Context,
    args: GetRouteTableArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetRouteTableResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routeTableName".to_string(), json!(args.route_table_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230401:getRouteTable", invoke_args, &opts).await?;

    Ok(GetRouteTableResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        disable_bgp_route_propagation: result.fields.get("disableBgpRoutePropagation").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        routes: result.fields.get("routes").cloned().map(serde_json::from_value).transpose()?,
        subnets: serde_json::from_value(result.fields.get("subnets").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
