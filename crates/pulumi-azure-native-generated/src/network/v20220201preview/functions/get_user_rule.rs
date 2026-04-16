use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a user rule.
#[derive(Default)]
pub struct GetUserRuleArgs {
    /// The name of the network manager Security Configuration.
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
pub struct GetUserRuleResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description for this rule.
    pub description: Option<String>,
    /// The destination port ranges.
    pub destination_port_ranges: Option<Vec<String>>,
    /// The destination address prefixes. CIDR or destination IP ranges.
    pub destinations: Option<Vec<network::v20220201preview::AddressPrefixItemResponse>>,
    /// Indicates if the traffic matched against the rule in inbound or outbound.
    pub direction: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
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
    pub source_port_ranges: Option<Vec<String>>,
    /// The CIDR or source IP ranges.
    pub sources: Option<Vec<network::v20220201preview::AddressPrefixItemResponse>>,
    /// The system metadata related to this resource.
    pub system_data: network::v20220201preview::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Gets a user rule.
pub async fn get_user_rule(
    ctx: &Context,
    args: GetUserRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetUserRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationName".to_string(), json!(args.configuration_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("ruleCollectionName".to_string(), json!(args.rule_collection_name));
    invoke_args.insert("ruleName".to_string(), json!(args.rule_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220201preview:getUserRule", invoke_args, &opts).await?;

    Ok(GetUserRuleResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        destination_port_ranges: result.fields.get("destinationPortRanges").cloned().map(serde_json::from_value).transpose()?,
        destinations: result.fields.get("destinations").cloned().map(serde_json::from_value).transpose()?,
        direction: serde_json::from_value(result.fields.get("direction").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
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
        source_port_ranges: result.fields.get("sourcePortRanges").cloned().map(serde_json::from_value).transpose()?,
        sources: result.fields.get("sources").cloned().map(serde_json::from_value).transpose()?,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
