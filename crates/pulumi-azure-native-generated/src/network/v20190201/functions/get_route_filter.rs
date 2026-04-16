use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified route filter.
#[derive(Default)]
pub struct GetRouteFilterArgs {
    /// Expands referenced express route bgp peering resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the route filter.
    pub route_filter_name: String,
}

/// Result of the function invocation.
pub struct GetRouteFilterResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// A collection of references to express route circuit peerings.
    pub peerings: Option<Vec<network::v20190201::ExpressRouteCircuitPeeringResponse>>,
    /// The provisioning state of the resource. Possible values are: 'Updating', 'Deleting', 'Succeeded' and 'Failed'.
    pub provisioning_state: String,
    /// Collection of RouteFilterRules contained within a route filter.
    pub rules: Option<Vec<network::v20190201::RouteFilterRuleResponse>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified route filter.
pub async fn get_route_filter(
    ctx: &Context,
    args: GetRouteFilterArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetRouteFilterResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routeFilterName".to_string(), json!(args.route_filter_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20190201:getRouteFilter", invoke_args, &opts).await?;

    Ok(GetRouteFilterResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        peerings: result.fields.get("peerings").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        rules: result.fields.get("rules").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
