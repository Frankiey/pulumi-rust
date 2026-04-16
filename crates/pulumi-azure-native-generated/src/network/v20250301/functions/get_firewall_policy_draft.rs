use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get a draft Firewall Policy.
#[derive(Default)]
pub struct GetFirewallPolicyDraftArgs {
    /// The name of the Firewall Policy.
    pub firewall_policy_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetFirewallPolicyDraftResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The parent firewall policy from which rules are inherited.
    pub base_policy: Option<network::v20250301::SubResourceResponse>,
    /// DNS Proxy Settings definition.
    pub dns_settings: Option<network::v20250301::DnsSettingsResponse>,
    /// Explicit Proxy Settings definition.
    pub explicit_proxy: Option<network::v20250301::ExplicitProxyResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// Insights on Firewall Policy.
    pub insights: Option<network::v20250301::FirewallPolicyInsightsResponse>,
    /// The configuration for Intrusion detection.
    pub intrusion_detection: Option<network::v20250301::FirewallPolicyIntrusionDetectionResponse>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The private IP addresses/IP ranges to which traffic will not be SNAT.
    pub snat: Option<network::v20250301::FirewallPolicySNATResponse>,
    /// SQL Settings definition.
    pub sql: Option<network::v20250301::FirewallPolicySQLResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Option<String>,
    /// ThreatIntel Whitelist for Firewall Policy.
    pub threat_intel_whitelist: Option<network::v20250301::FirewallPolicyThreatIntelWhitelistResponse>,
    /// Resource type.
    pub type_: String,
}

/// Get a draft Firewall Policy.
pub async fn get_firewall_policy_draft(
    ctx: &Context,
    args: GetFirewallPolicyDraftArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetFirewallPolicyDraftResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("firewallPolicyName".to_string(), json!(args.firewall_policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getFirewallPolicyDraft", invoke_args, &opts).await?;

    Ok(GetFirewallPolicyDraftResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        base_policy: result.fields.get("basePolicy").cloned().map(serde_json::from_value).transpose()?,
        dns_settings: result.fields.get("dnsSettings").cloned().map(serde_json::from_value).transpose()?,
        explicit_proxy: result.fields.get("explicitProxy").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        insights: result.fields.get("insights").cloned().map(serde_json::from_value).transpose()?,
        intrusion_detection: result.fields.get("intrusionDetection").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        snat: result.fields.get("snat").cloned().map(serde_json::from_value).transpose()?,
        sql: result.fields.get("sql").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        threat_intel_mode: result.fields.get("threatIntelMode").cloned().map(serde_json::from_value).transpose()?,
        threat_intel_whitelist: result.fields.get("threatIntelWhitelist").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
