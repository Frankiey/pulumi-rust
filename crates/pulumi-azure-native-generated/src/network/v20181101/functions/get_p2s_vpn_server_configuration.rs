use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a P2SVpnServerConfiguration.
#[derive(Default)]
pub struct GetP2sVpnServerConfigurationArgs {
    /// The name of the P2SVpnServerConfiguration.
    pub p2s_vpn_server_configuration_name: String,
    /// The resource group name of the P2SVpnServerConfiguration.
    pub resource_group_name: String,
    /// The name of the VirtualWan.
    pub virtual_wan_name: String,
}

/// Result of the function invocation.
pub struct GetP2sVpnServerConfigurationResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Parameters for P2SVpnServerConfiguration
    pub properties: network::v20181101::P2SVpnServerConfigurationPropertiesResponse,
}

/// Retrieves the details of a P2SVpnServerConfiguration.
pub async fn get_p2s_vpn_server_configuration(
    ctx: &Context,
    args: GetP2sVpnServerConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetP2sVpnServerConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("p2SVpnServerConfigurationName".to_string(), json!(args.p2s_vpn_server_configuration_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualWanName".to_string(), json!(args.virtual_wan_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20181101:getP2sVpnServerConfiguration", invoke_args, &opts).await?;

    Ok(GetP2sVpnServerConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
    })
}
