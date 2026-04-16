use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the NSP logging configuration.
/// 
/// Uses Azure REST API version 2024-06-01-preview.
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
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// Properties of the NSP logging configuration.
    pub properties: network::NspLoggingConfigurationPropertiesResponse,
    /// Resource type.
    pub type_: String,
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
    let result = ctx.invoke("azure-native:network:getNetworkSecurityPerimeterLoggingConfiguration", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityPerimeterLoggingConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
