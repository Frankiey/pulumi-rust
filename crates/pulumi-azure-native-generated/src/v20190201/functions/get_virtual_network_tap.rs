use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets information about the specified virtual network tap.
#[derive(Default)]
pub struct GetVirtualNetworkTapArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of virtual network tap.
    pub tap_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkTapResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The reference to the private IP address on the internal Load Balancer that will receive the tap
    pub destination_load_balancer_front_end_ip_configuration: Option<network::v20190201::FrontendIPConfigurationResponse>,
    /// The reference to the private IP Address of the collector nic that will receive the tap
    pub destination_network_interface_ip_configuration: Option<network::v20190201::NetworkInterfaceIPConfigurationResponse>,
    /// The VXLAN destination port that will receive the tapped traffic.
    pub destination_port: Option<i64>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Specifies the list of resource IDs for the network interface IP configuration that needs to be tapped.
    pub network_interface_tap_configurations: Vec<network::v20190201::NetworkInterfaceTapConfigurationResponse>,
    /// The provisioning state of the virtual network tap. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: String,
    /// The resourceGuid property of the virtual network tap.
    pub resource_guid: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets information about the specified virtual network tap.
pub async fn get_virtual_network_tap(
    ctx: &Context,
    args: GetVirtualNetworkTapArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkTapResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("tapName".to_string(), json!(args.tap_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20190201:getVirtualNetworkTap", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkTapResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        destination_load_balancer_front_end_ip_configuration: result.fields.get("destinationLoadBalancerFrontEndIPConfiguration").cloned().map(serde_json::from_value).transpose()?,
        destination_network_interface_ip_configuration: result.fields.get("destinationNetworkInterfaceIPConfiguration").cloned().map(serde_json::from_value).transpose()?,
        destination_port: result.fields.get("destinationPort").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_interface_tap_configurations: serde_json::from_value(result.fields.get("networkInterfaceTapConfigurations").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
