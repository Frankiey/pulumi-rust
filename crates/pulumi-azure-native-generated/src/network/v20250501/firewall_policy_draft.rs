use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// FirewallPolicy Resource.
pub struct FirewallPolicyDraftArgs {
    /// The parent firewall policy from which rules are inherited.
    pub base_policy: Option<Input<network::v20250501::SubResourceArgs>>,
    /// DNS Proxy Settings definition.
    pub dns_settings: Option<Input<network::v20250501::DnsSettingsArgs>>,
    /// Explicit Proxy Settings definition.
    pub explicit_proxy: Option<Input<network::v20250501::ExplicitProxyArgs>>,
    /// The name of the Firewall Policy.
    pub firewall_policy_name: Input<String>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Insights on Firewall Policy.
    pub insights: Option<Input<network::v20250501::FirewallPolicyInsightsArgs>>,
    /// The configuration for Intrusion detection.
    pub intrusion_detection: Option<Input<network::v20250501::FirewallPolicyIntrusionDetectionArgs>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The private IP addresses/IP ranges to which traffic will not be SNAT.
    pub snat: Option<Input<network::v20250501::FirewallPolicySNATArgs>>,
    /// SQL Settings definition.
    pub sql: Option<Input<network::v20250501::FirewallPolicySQLArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Option<Input<serde_json::Value>>,
    /// ThreatIntel Whitelist for Firewall Policy.
    pub threat_intel_whitelist: Option<Input<network::v20250501::FirewallPolicyThreatIntelWhitelistArgs>>,
}

/// FirewallPolicy Resource.
pub struct FirewallPolicyDraft {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The parent firewall policy from which rules are inherited.
    pub base_policy: Output<serde_json::Value>,
    /// DNS Proxy Settings definition.
    pub dns_settings: Output<serde_json::Value>,
    /// Explicit Proxy Settings definition.
    pub explicit_proxy: Output<serde_json::Value>,
    /// Insights on Firewall Policy.
    pub insights: Output<serde_json::Value>,
    /// The configuration for Intrusion detection.
    pub intrusion_detection: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The private IP addresses/IP ranges to which traffic will not be SNAT.
    pub snat: Output<serde_json::Value>,
    /// SQL Settings definition.
    pub sql: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Output<serde_json::Value>,
    /// ThreatIntel Whitelist for Firewall Policy.
    pub threat_intel_whitelist: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl FirewallPolicyDraft {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250501:FirewallPolicyDraft";

    /// Create a new FirewallPolicyDraft resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: FirewallPolicyDraftArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.base_policy {
            pulumi_sdk::resolve_input("basePolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dns_settings {
            pulumi_sdk::resolve_input("dnsSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.explicit_proxy {
            pulumi_sdk::resolve_input("explicitProxy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("firewallPolicyName", args.firewall_policy_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.insights {
            pulumi_sdk::resolve_input("insights", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.intrusion_detection {
            pulumi_sdk::resolve_input("intrusionDetection", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.snat {
            pulumi_sdk::resolve_input("snat", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sql {
            pulumi_sdk::resolve_input("sql", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.threat_intel_mode {
            pulumi_sdk::resolve_input("threatIntelMode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.threat_intel_whitelist {
            pulumi_sdk::resolve_input("threatIntelWhitelist", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            base_policy: registered.outputs.get("basePolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dns_settings: registered.outputs.get("dnsSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            explicit_proxy: registered.outputs.get("explicitProxy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            insights: registered.outputs.get("insights")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            intrusion_detection: registered.outputs.get("intrusionDetection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            snat: registered.outputs.get("snat")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sql: registered.outputs.get("sql")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            threat_intel_mode: registered.outputs.get("threatIntelMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            threat_intel_whitelist: registered.outputs.get("threatIntelWhitelist")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
