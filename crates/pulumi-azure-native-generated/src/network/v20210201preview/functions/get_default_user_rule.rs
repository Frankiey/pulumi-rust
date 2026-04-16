use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a user rule.
#[derive(Default)]
pub struct GetDefaultUserRuleArgs {
    /// The name of the network manager security Configuration.
    pub configuration_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the network manager security Configuration rule collection.
    pub rule_collection_name: String,
    /// The name of the rule.
    pub rule_name: String,
}

/// Result of the function invocation.
pub struct GetDefaultUserRuleResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description for this rule. Restricted to 140 chars.
    pub description: String,
    /// The destination port ranges.
    pub destination_port_ranges: Vec<String>,
    /// The destination address prefixes. CIDR or destination IP ranges.
    pub destinations: Vec<network::v20210201preview::AddressPrefixItemResponse>,
    /// Indicates if the traffic matched against the rule in inbound or outbound.
    pub direction: String,
    /// A friendly name for the rule.
    pub display_name: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Default rule flag.
    pub flag: Option<String>,
    /// Resource ID.
    pub id: String,
    /// Whether the rule is custom or default.
    pub kind: String,
    /// Resource name.
    pub name: String,
    /// Network protocol this rule applies to.
    pub protocol: String,
    /// The provisioning state of the security configuration user rule resource.
    pub provisioning_state: String,
    /// The source port ranges.
    pub source_port_ranges: Vec<String>,
    /// The CIDR or source IP ranges.
    pub sources: Vec<network::v20210201preview::AddressPrefixItemResponse>,
    /// The system metadata related to this resource.
    pub system_data: network::v20210201preview::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Gets a user rule.
pub async fn get_default_user_rule(
    ctx: &Context,
    args: GetDefaultUserRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetDefaultUserRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationName".to_string(), json!(args.configuration_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("ruleCollectionName".to_string(), json!(args.rule_collection_name));
    invoke_args.insert("ruleName".to_string(), json!(args.rule_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20210201preview:getDefaultUserRule", invoke_args, &opts).await?;

    Ok(GetDefaultUserRuleResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: serde_json::from_value(result.fields.get("description").cloned().unwrap_or_default())?
            ,
        destination_port_ranges: serde_json::from_value(result.fields.get("destinationPortRanges").cloned().unwrap_or_default())?
            ,
        destinations: serde_json::from_value(result.fields.get("destinations").cloned().unwrap_or_default())?
            ,
        direction: serde_json::from_value(result.fields.get("direction").cloned().unwrap_or_default())?
            ,
        display_name: serde_json::from_value(result.fields.get("displayName").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        flag: result.fields.get("flag").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        kind: serde_json::from_value(result.fields.get("kind").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        protocol: serde_json::from_value(result.fields.get("protocol").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        source_port_ranges: serde_json::from_value(result.fields.get("sourcePortRanges").cloned().unwrap_or_default())?
            ,
        sources: serde_json::from_value(result.fields.get("sources").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
