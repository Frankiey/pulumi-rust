use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified FirewallPolicyRuleCollectionGroup.
/// 
/// Uses Azure REST API version 2024-05-01.
#[derive(Default)]
pub struct GetFirewallPolicyRuleCollectionGroupArgs {
    /// The name of the Firewall Policy.
    pub firewall_policy_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the FirewallPolicyRuleCollectionGroup.
    pub rule_collection_group_name: String,
}

/// Result of the function invocation.
pub struct GetFirewallPolicyRuleCollectionGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Priority of the Firewall Policy Rule Collection Group resource.
    pub priority: Option<i64>,
    /// The provisioning state of the firewall policy rule collection group resource.
    pub provisioning_state: String,
    /// Group of Firewall Policy rule collections.
    pub rule_collections: Option<Vec<serde_json::Value>>,
    /// A read-only string that represents the size of the FirewallPolicyRuleCollectionGroupProperties in MB. (ex 1.2MB)
    pub size: String,
    /// Rule Group type.
    pub type_: String,
}

/// Gets the specified FirewallPolicyRuleCollectionGroup.
pub async fn get_firewall_policy_rule_collection_group(
    ctx: &Context,
    args: GetFirewallPolicyRuleCollectionGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetFirewallPolicyRuleCollectionGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("firewallPolicyName".to_string(), json!(args.firewall_policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("ruleCollectionGroupName".to_string(), json!(args.rule_collection_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:getFirewallPolicyRuleCollectionGroup", invoke_args, &opts).await?;

    Ok(GetFirewallPolicyRuleCollectionGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        priority: result.fields.get("priority").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        rule_collections: result.fields.get("ruleCollections").cloned().map(serde_json::from_value).transpose()?,
        size: serde_json::from_value(result.fields.get("size").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
