use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a network manager security admin configuration rule collection.
#[derive(Default)]
pub struct GetAdminRuleCollectionArgs {
    /// The name of the network manager Security Configuration.
    pub configuration_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the network manager security Configuration rule collection.
    pub rule_collection_name: String,
}

/// Result of the function invocation.
pub struct GetAdminRuleCollectionResult {
    /// Groups for configuration
    pub applies_to_groups: Vec<network::v20230201::NetworkManagerSecurityGroupItemResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description of the admin rule collection.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Unique identifier for this resource.
    pub resource_guid: String,
    /// The system metadata related to this resource.
    pub system_data: network::v20230201::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Gets a network manager security admin configuration rule collection.
pub async fn get_admin_rule_collection(
    ctx: &Context,
    args: GetAdminRuleCollectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetAdminRuleCollectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationName".to_string(), json!(args.configuration_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("ruleCollectionName".to_string(), json!(args.rule_collection_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230201:getAdminRuleCollection", invoke_args, &opts).await?;

    Ok(GetAdminRuleCollectionResult {
        applies_to_groups: serde_json::from_value(result.fields.get("appliesToGroups").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
