use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves a network manager security admin configuration.
#[derive(Default)]
pub struct GetSecurityAdminConfigurationArgs {
    /// The name of the network manager Security Configuration.
    pub configuration_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetSecurityAdminConfigurationResult {
    /// Enum list of network intent policy based services.
    pub apply_on_network_intent_policy_based_services: Option<Vec<String>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description of the security configuration.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// The system metadata related to this resource.
    pub system_data: network::v20220501::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Retrieves a network manager security admin configuration.
pub async fn get_security_admin_configuration(
    ctx: &Context,
    args: GetSecurityAdminConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetSecurityAdminConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationName".to_string(), json!(args.configuration_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220501:getSecurityAdminConfiguration", invoke_args, &opts).await?;

    Ok(GetSecurityAdminConfigurationResult {
        apply_on_network_intent_policy_based_services: result.fields.get("applyOnNetworkIntentPolicyBasedServices").cloned().map(serde_json::from_value).transpose()?,
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
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
