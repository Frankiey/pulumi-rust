use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a VpnServerConfiguration.
#[derive(Default)]
pub struct GetVpnServerConfigurationArgs {
    /// The resource group name of the VpnServerConfiguration.
    pub resource_group_name: String,
    /// The name of the VpnServerConfiguration being retrieved.
    pub vpn_server_configuration_name: String,
}

/// Result of the function invocation.
pub struct GetVpnServerConfigurationResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Properties of the P2SVpnServer configuration.
    pub properties: network::v20201101::VpnServerConfigurationPropertiesResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Retrieves the details of a VpnServerConfiguration.
pub async fn get_vpn_server_configuration(
    ctx: &Context,
    args: GetVpnServerConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVpnServerConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("vpnServerConfigurationName".to_string(), json!(args.vpn_server_configuration_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20201101:getVpnServerConfiguration", invoke_args, &opts).await?;

    Ok(GetVpnServerConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
