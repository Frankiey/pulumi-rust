use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Firewall Policy.
#[derive(Default)]
pub struct GetFirewallPolicyArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the Firewall Policy.
    pub firewall_policy_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetFirewallPolicyResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The parent firewall policy from which rules are inherited.
    pub base_policy: Option<network::v20220501::SubResourceResponse>,
    /// List of references to Child Firewall Policies.
    pub child_policies: Vec<network::v20220501::SubResourceResponse>,
    /// DNS Proxy Settings definition.
    pub dns_settings: Option<network::v20220501::DnsSettingsResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Explicit Proxy Settings definition.
    pub explicit_proxy: Option<network::v20220501::ExplicitProxyResponse>,
    /// List of references to Azure Firewalls that this Firewall Policy is associated with.
    pub firewalls: Vec<network::v20220501::SubResourceResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// The identity of the firewall policy.
    pub identity: Option<network::v20220501::ManagedServiceIdentityResponse>,
    /// Insights on Firewall Policy.
    pub insights: Option<network::v20220501::FirewallPolicyInsightsResponse>,
    /// The configuration for Intrusion detection.
    pub intrusion_detection: Option<network::v20220501::FirewallPolicyIntrusionDetectionResponse>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the firewall policy resource.
    pub provisioning_state: String,
    /// List of references to FirewallPolicyRuleCollectionGroups.
    pub rule_collection_groups: Vec<network::v20220501::SubResourceResponse>,
    /// The Firewall Policy SKU.
    pub sku: Option<network::v20220501::FirewallPolicySkuResponse>,
    /// The private IP addresses/IP ranges to which traffic will not be SNAT.
    pub snat: Option<network::v20220501::FirewallPolicySNATResponse>,
    /// SQL Settings definition.
    pub sql: Option<network::v20220501::FirewallPolicySQLResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Option<String>,
    /// ThreatIntel Whitelist for Firewall Policy.
    pub threat_intel_whitelist: Option<network::v20220501::FirewallPolicyThreatIntelWhitelistResponse>,
    /// TLS Configuration definition.
    pub transport_security: Option<network::v20220501::FirewallPolicyTransportSecurityResponse>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified Firewall Policy.
pub async fn get_firewall_policy(
    ctx: &Context,
    args: GetFirewallPolicyArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetFirewallPolicyResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("firewallPolicyName".to_string(), json!(args.firewall_policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220501:getFirewallPolicy", invoke_args, &opts).await?;

    Ok(GetFirewallPolicyResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        base_policy: result.fields.get("basePolicy").cloned().map(serde_json::from_value).transpose()?,
        child_policies: serde_json::from_value(result.fields.get("childPolicies").cloned().unwrap_or_default())?
            ,
        dns_settings: result.fields.get("dnsSettings").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        explicit_proxy: result.fields.get("explicitProxy").cloned().map(serde_json::from_value).transpose()?,
        firewalls: serde_json::from_value(result.fields.get("firewalls").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        insights: result.fields.get("insights").cloned().map(serde_json::from_value).transpose()?,
        intrusion_detection: result.fields.get("intrusionDetection").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        rule_collection_groups: serde_json::from_value(result.fields.get("ruleCollectionGroups").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        snat: result.fields.get("snat").cloned().map(serde_json::from_value).transpose()?,
        sql: result.fields.get("sql").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        threat_intel_mode: result.fields.get("threatIntelMode").cloned().map(serde_json::from_value).transpose()?,
        threat_intel_whitelist: result.fields.get("threatIntelWhitelist").cloned().map(serde_json::from_value).transpose()?,
        transport_security: result.fields.get("transportSecurity").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
