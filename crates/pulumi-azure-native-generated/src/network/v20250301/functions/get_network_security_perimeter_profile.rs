use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified NSP profile.
#[derive(Default)]
pub struct GetNetworkSecurityPerimeterProfileArgs {
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: String,
    /// The name of the NSP profile.
    pub profile_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkSecurityPerimeterProfileResult {
    /// Version number that increases with every update to access rules within the profile.
    pub access_rules_version: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Version number that increases with every update to diagnostic settings within the profile.
    pub diagnostic_settings_version: String,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::v20250301::SystemDataResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the specified NSP profile.
pub async fn get_network_security_perimeter_profile(
    ctx: &Context,
    args: GetNetworkSecurityPerimeterProfileArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkSecurityPerimeterProfileResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkSecurityPerimeterName".to_string(), json!(args.network_security_perimeter_name));
    invoke_args.insert("profileName".to_string(), json!(args.profile_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getNetworkSecurityPerimeterProfile", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityPerimeterProfileResult {
        access_rules_version: serde_json::from_value(result.fields.get("accessRulesVersion").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        diagnostic_settings_version: serde_json::from_value(result.fields.get("diagnosticSettingsVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
