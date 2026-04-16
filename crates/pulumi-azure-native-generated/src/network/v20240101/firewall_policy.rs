use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// FirewallPolicy Resource.
pub struct FirewallPolicyArgs {
    /// The parent firewall policy from which rules are inherited.
    pub base_policy: Option<Input<network::v20240101::SubResourceArgs>>,
    /// DNS Proxy Settings definition.
    pub dns_settings: Option<Input<network::v20240101::DnsSettingsArgs>>,
    /// Explicit Proxy Settings definition.
    pub explicit_proxy: Option<Input<network::v20240101::ExplicitProxyArgs>>,
    /// The name of the Firewall Policy.
    pub firewall_policy_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The identity of the firewall policy.
    pub identity: Option<Input<network::v20240101::ManagedServiceIdentityArgs>>,
    /// Insights on Firewall Policy.
    pub insights: Option<Input<network::v20240101::FirewallPolicyInsightsArgs>>,
    /// The configuration for Intrusion detection.
    pub intrusion_detection: Option<Input<network::v20240101::FirewallPolicyIntrusionDetectionArgs>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The Firewall Policy SKU.
    pub sku: Option<Input<network::v20240101::FirewallPolicySkuArgs>>,
    /// The private IP addresses/IP ranges to which traffic will not be SNAT.
    pub snat: Option<Input<network::v20240101::FirewallPolicySNATArgs>>,
    /// SQL Settings definition.
    pub sql: Option<Input<network::v20240101::FirewallPolicySQLArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Option<Input<serde_json::Value>>,
    /// ThreatIntel Whitelist for Firewall Policy.
    pub threat_intel_whitelist: Option<Input<network::v20240101::FirewallPolicyThreatIntelWhitelistArgs>>,
    /// TLS Configuration definition.
    pub transport_security: Option<Input<network::v20240101::FirewallPolicyTransportSecurityArgs>>,
}

/// FirewallPolicy Resource.
pub struct FirewallPolicy {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The parent firewall policy from which rules are inherited.
    pub base_policy: Output<serde_json::Value>,
    /// List of references to Child Firewall Policies.
    pub child_policies: Output<serde_json::Value>,
    /// DNS Proxy Settings definition.
    pub dns_settings: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Explicit Proxy Settings definition.
    pub explicit_proxy: Output<serde_json::Value>,
    /// List of references to Azure Firewalls that this Firewall Policy is associated with.
    pub firewalls: Output<serde_json::Value>,
    /// The identity of the firewall policy.
    pub identity: Output<serde_json::Value>,
    /// Insights on Firewall Policy.
    pub insights: Output<serde_json::Value>,
    /// The configuration for Intrusion detection.
    pub intrusion_detection: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the firewall policy resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// List of references to FirewallPolicyRuleCollectionGroups.
    pub rule_collection_groups: Output<serde_json::Value>,
    /// A read-only string that represents the size of the FirewallPolicyPropertiesFormat in MB. (ex 0.5MB)
    pub size: Output<serde_json::Value>,
    /// The Firewall Policy SKU.
    pub sku: Output<serde_json::Value>,
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
    /// TLS Configuration definition.
    pub transport_security: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl FirewallPolicy {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240101:FirewallPolicy";

    /// Create a new FirewallPolicy resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: FirewallPolicyArgs,
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
        if let Some(v) = args.firewall_policy_name {
            pulumi_sdk::resolve_input("firewallPolicyName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
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
        if let Some(v) = args.transport_security {
            pulumi_sdk::resolve_input("transportSecurity", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            child_policies: registered.outputs.get("childPolicies")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dns_settings: registered.outputs.get("dnsSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            explicit_proxy: registered.outputs.get("explicitProxy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            firewalls: registered.outputs.get("firewalls")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
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
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            rule_collection_groups: registered.outputs.get("ruleCollectionGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            size: registered.outputs.get("size")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
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
            transport_security: registered.outputs.get("transportSecurity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
