use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified FirewallPolicyRuleGroup.
#[derive(Default)]
pub struct GetFirewallPolicyRuleGroupArgs {
    /// The name of the Firewall Policy.
    pub firewall_policy_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the FirewallPolicyRuleGroup.
    pub rule_group_name: String,
}

/// Result of the function invocation.
pub struct GetFirewallPolicyRuleGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Priority of the Firewall Policy Rule Group resource.
    pub priority: Option<i64>,
    /// The provisioning state of the firewall policy rule group resource.
    pub provisioning_state: String,
    /// Group of Firewall Policy rules.
    pub rules: Option<Vec<serde_json::Value>>,
    /// Rule Group type.
    pub type_: String,
}

/// Gets the specified FirewallPolicyRuleGroup.
pub async fn get_firewall_policy_rule_group(
    ctx: &Context,
    args: GetFirewallPolicyRuleGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetFirewallPolicyRuleGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("firewallPolicyName".to_string(), json!(args.firewall_policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("ruleGroupName".to_string(), json!(args.rule_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20191201:getFirewallPolicyRuleGroup", invoke_args, &opts).await?;

    Ok(GetFirewallPolicyRuleGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        priority: result.fields.get("priority").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        rules: result.fields.get("rules").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
