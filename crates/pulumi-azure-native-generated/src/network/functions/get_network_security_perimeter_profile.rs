use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified NSP profile.
/// 
/// Uses Azure REST API version 2024-06-01-preview.
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
    /// Resource ID.
    pub id: String,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
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
    let result = ctx.invoke("azure-native:network:getNetworkSecurityPerimeterProfile", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityPerimeterProfileResult {
        access_rules_version: serde_json::from_value(result.fields.get("accessRulesVersion").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        diagnostic_settings_version: serde_json::from_value(result.fields.get("diagnosticSettingsVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
