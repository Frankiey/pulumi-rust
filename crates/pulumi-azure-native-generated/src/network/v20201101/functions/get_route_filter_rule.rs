use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified rule from a route filter.
#[derive(Default)]
pub struct GetRouteFilterRuleArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the route filter.
    pub route_filter_name: String,
    /// The name of the rule.
    pub rule_name: String,
}

/// Result of the function invocation.
pub struct GetRouteFilterRuleResult {
    /// The access type of the rule.
    pub access: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The collection for bgp community values to filter on. e.g. ['12076:5010','12076:5020'].
    pub communities: Vec<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the route filter rule resource.
    pub provisioning_state: String,
    /// The rule type of the rule.
    pub route_filter_rule_type: String,
}

/// Gets the specified rule from a route filter.
pub async fn get_route_filter_rule(
    ctx: &Context,
    args: GetRouteFilterRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetRouteFilterRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routeFilterName".to_string(), json!(args.route_filter_name));
    invoke_args.insert("ruleName".to_string(), json!(args.rule_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20201101:getRouteFilterRule", invoke_args, &opts).await?;

    Ok(GetRouteFilterRuleResult {
        access: serde_json::from_value(result.fields.get("access").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        communities: serde_json::from_value(result.fields.get("communities").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        route_filter_rule_type: serde_json::from_value(result.fields.get("routeFilterRuleType").cloned().unwrap_or_default())?
            ,
    })
}
