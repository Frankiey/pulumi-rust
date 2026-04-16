use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Defines web application firewall policy.
pub struct WebApplicationFirewallPolicyArgs {
    /// The custom rules inside the policy.
    pub custom_rules: Option<Vec<Input<network::v20240501::WebApplicationFirewallCustomRuleArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// Describes the managedRules structure.
    pub managed_rules: Input<network::v20240501::ManagedRulesDefinitionArgs>,
    /// The name of the policy.
    pub policy_name: Option<Input<String>>,
    /// The PolicySettings for policy.
    pub policy_settings: Option<Input<network::v20240501::PolicySettingsArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Defines web application firewall policy.
pub struct WebApplicationFirewallPolicy {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// A collection of references to application gateway for containers.
    pub application_gateway_for_containers: Output<serde_json::Value>,
    /// A collection of references to application gateways.
    pub application_gateways: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The custom rules inside the policy.
    pub custom_rules: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// A collection of references to application gateway http listeners.
    pub http_listeners: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Describes the managedRules structure.
    pub managed_rules: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// A collection of references to application gateway path rules.
    pub path_based_rules: Output<serde_json::Value>,
    /// The PolicySettings for policy.
    pub policy_settings: Output<serde_json::Value>,
    /// The provisioning state of the web application firewall policy resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource status of the policy.
    pub resource_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl WebApplicationFirewallPolicy {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240501:WebApplicationFirewallPolicy";

    /// Create a new WebApplicationFirewallPolicy resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: WebApplicationFirewallPolicyArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.custom_rules {
            pulumi_sdk::resolve_input_vec("customRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("managedRules", args.managed_rules, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.policy_name {
            pulumi_sdk::resolve_input("policyName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.policy_settings {
            pulumi_sdk::resolve_input("policySettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            application_gateway_for_containers: registered.outputs.get("applicationGatewayForContainers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            application_gateways: registered.outputs.get("applicationGateways")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            custom_rules: registered.outputs.get("customRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            http_listeners: registered.outputs.get("httpListeners")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            managed_rules: registered.outputs.get("managedRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            path_based_rules: registered.outputs.get("pathBasedRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            policy_settings: registered.outputs.get("policySettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_state: registered.outputs.get("resourceState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
