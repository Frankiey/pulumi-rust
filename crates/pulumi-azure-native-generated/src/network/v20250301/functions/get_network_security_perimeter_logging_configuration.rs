use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the NSP logging configuration.
#[derive(Default)]
pub struct GetNetworkSecurityPerimeterLoggingConfigurationArgs {
    /// The name of the NSP logging configuration. Accepts 'instance' as name.
    pub logging_configuration_name: String,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkSecurityPerimeterLoggingConfigurationResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The log categories to enable in the NSP logging configuration.
    pub enabled_log_categories: Option<Vec<String>>,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::v20250301::SystemDataResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
    /// The version of the NSP logging configuration.
    pub version: Option<String>,
}

/// Gets the NSP logging configuration.
pub async fn get_network_security_perimeter_logging_configuration(
    ctx: &Context,
    args: GetNetworkSecurityPerimeterLoggingConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkSecurityPerimeterLoggingConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("loggingConfigurationName".to_string(), json!(args.logging_configuration_name));
    invoke_args.insert("networkSecurityPerimeterName".to_string(), json!(args.network_security_perimeter_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getNetworkSecurityPerimeterLoggingConfiguration", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityPerimeterLoggingConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        enabled_log_categories: result.fields.get("enabledLogCategories").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        version: result.fields.get("version").cloned().map(serde_json::from_value).transpose()?,
    })
}
