use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get the specified tap configuration on a network interface.
#[derive(Default)]
pub struct GetNetworkInterfaceTapConfigurationArgs {
    /// The name of the network interface.
    pub network_interface_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the tap configuration.
    pub tap_configuration_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkInterfaceTapConfigurationResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the network interface tap configuration resource.
    pub provisioning_state: String,
    /// Sub Resource type.
    pub type_: String,
    /// The reference of the Virtual Network Tap resource.
    pub virtual_network_tap: Option<network::v20190801::VirtualNetworkTapResponse>,
}

/// Get the specified tap configuration on a network interface.
pub async fn get_network_interface_tap_configuration(
    ctx: &Context,
    args: GetNetworkInterfaceTapConfigurationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkInterfaceTapConfigurationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkInterfaceName".to_string(), json!(args.network_interface_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("tapConfigurationName".to_string(), json!(args.tap_configuration_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20190801:getNetworkInterfaceTapConfiguration", invoke_args, &opts).await?;

    Ok(GetNetworkInterfaceTapConfigurationResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_network_tap: result.fields.get("virtualNetworkTap").cloned().map(serde_json::from_value).transpose()?,
    })
}
