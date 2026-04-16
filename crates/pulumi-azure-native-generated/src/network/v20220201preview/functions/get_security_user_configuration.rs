use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves a network manager security user configuration.
#[derive(Default)]
pub struct GetSecurityUserConfigurationArgs {
    /// The name of the network manager Security Configuration.
    pub configuration_name: String,
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetSecurityUserConfigurationResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Flag if need to delete existing network security groups.
    pub delete_existing_ns_gs: Option<String>,
    /// A description of the security user configuration.
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
    pub system_data: network::v20220201preview::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Retrieves a network manager security user configuration.
pub async fn get_security_user_configuration(
    ctx: &Context,
    args: GetSecurityUserConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetSecurityUserConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("configurationName".to_string(), json!(args.configuration_name));
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220201preview:getSecurityUserConfiguration", invoke_args, &opts).await?;

    Ok(GetSecurityUserConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        delete_existing_ns_gs: result.fields.get("deleteExistingNSGs").cloned().map(serde_json::from_value).transpose()?,
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
