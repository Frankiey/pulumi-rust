use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Rule Group resource.
pub struct FirewallPolicyRuleGroupArgs {
    /// The name of the Firewall Policy.
    pub firewall_policy_name: Input<String>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// Priority of the Firewall Policy Rule Group resource.
    pub priority: Option<Input<i64>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the FirewallPolicyRuleGroup.
    pub rule_group_name: Option<Input<String>>,
    /// Group of Firewall Policy rules.
    pub rules: Option<Vec<Input<serde_json::Value>>>,
}

/// Rule Group resource.
pub struct FirewallPolicyRuleGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// Priority of the Firewall Policy Rule Group resource.
    pub priority: Output<serde_json::Value>,
    /// The provisioning state of the firewall policy rule group resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Group of Firewall Policy rules.
    pub rules: Output<serde_json::Value>,
    /// Rule Group type.
    pub type_: Output<serde_json::Value>,
}

impl FirewallPolicyRuleGroup {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190901:FirewallPolicyRuleGroup";

    /// Create a new FirewallPolicyRuleGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: FirewallPolicyRuleGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("firewallPolicyName", args.firewall_policy_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.priority {
            pulumi_sdk::resolve_input("priority", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.rule_group_name {
            pulumi_sdk::resolve_input("ruleGroupName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.rules {
            pulumi_sdk::resolve_input_vec("rules", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            priority: registered.outputs.get("priority")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            rules: registered.outputs.get("rules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
