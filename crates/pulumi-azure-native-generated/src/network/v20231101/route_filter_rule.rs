use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Route Filter Rule Resource.
pub struct RouteFilterRuleArgs {
    /// The access type of the rule.
    pub access: Input<serde_json::Value>,
    /// The collection for bgp community values to filter on. e.g. ['12076:5010','12076:5020'].
    pub communities: Vec<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the route filter.
    pub route_filter_name: Input<String>,
    /// The rule type of the rule.
    pub route_filter_rule_type: Input<serde_json::Value>,
    /// The name of the route filter rule.
    pub rule_name: Option<Input<String>>,
}

/// Route Filter Rule Resource.
pub struct RouteFilterRule {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The access type of the rule.
    pub access: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The collection for bgp community values to filter on. e.g. ['12076:5010','12076:5020'].
    pub communities: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the route filter rule resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The rule type of the rule.
    pub route_filter_rule_type: Output<serde_json::Value>,
}

impl RouteFilterRule {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20231101:RouteFilterRule";

    /// Create a new RouteFilterRule resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: RouteFilterRuleArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("access", args.access, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input_vec("communities", args.communities, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("routeFilterName", args.route_filter_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("routeFilterRuleType", args.route_filter_rule_type, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.rule_name {
            pulumi_sdk::resolve_input("ruleName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            access: registered.outputs.get("access")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            communities: registered.outputs.get("communities")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_filter_rule_type: registered.outputs.get("routeFilterRuleType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
