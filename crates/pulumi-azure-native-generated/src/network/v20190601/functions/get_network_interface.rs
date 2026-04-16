use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets information about the specified network interface.
#[derive(Default)]
pub struct GetNetworkInterfaceArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the network interface.
    pub network_interface_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkInterfaceResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The DNS settings in network interface.
    pub dns_settings: Option<network::v20190601::NetworkInterfaceDnsSettingsResponse>,
    /// If the network interface is accelerated networking enabled.
    pub enable_accelerated_networking: Option<bool>,
    /// Indicates whether IP forwarding is enabled on this network interface.
    pub enable_ip_forwarding: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// A list of references to linked BareMetal resources.
    pub hosted_workloads: Vec<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// A list of IPConfigurations of the network interface.
    pub ip_configurations: Option<Vec<network::v20190601::NetworkInterfaceIPConfigurationResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// The MAC address of the network interface.
    pub mac_address: Option<String>,
    /// Resource name.
    pub name: String,
    /// The reference of the NetworkSecurityGroup resource.
    pub network_security_group: Option<network::v20190601::NetworkSecurityGroupResponse>,
    /// Gets whether this is a primary network interface on a virtual machine.
    pub primary: Option<bool>,
    /// A reference to the private endpoint to which the network interface is linked.
    pub private_endpoint: network::v20190601::PrivateEndpointResponse,
    /// The provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// The resource GUID property of the network interface resource.
    pub resource_guid: Option<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// A list of TapConfigurations of the network interface.
    pub tap_configurations: Option<Vec<network::v20190601::NetworkInterfaceTapConfigurationResponse>>,
    /// Resource type.
    pub type_: String,
    /// The reference of a virtual machine.
    pub virtual_machine: network::v20190601::SubResourceResponse,
}

/// Gets information about the specified network interface.
pub async fn get_network_interface(
    ctx: &Context,
    args: GetNetworkInterfaceArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkInterfaceResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("networkInterfaceName".to_string(), json!(args.network_interface_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20190601:getNetworkInterface", invoke_args, &opts).await?;

    Ok(GetNetworkInterfaceResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        dns_settings: result.fields.get("dnsSettings").cloned().map(serde_json::from_value).transpose()?,
        enable_accelerated_networking: result.fields.get("enableAcceleratedNetworking").cloned().map(serde_json::from_value).transpose()?,
        enable_ip_forwarding: result.fields.get("enableIPForwarding").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        hosted_workloads: serde_json::from_value(result.fields.get("hostedWorkloads").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        mac_address: result.fields.get("macAddress").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_security_group: result.fields.get("networkSecurityGroup").cloned().map(serde_json::from_value).transpose()?,
        primary: result.fields.get("primary").cloned().map(serde_json::from_value).transpose()?,
        private_endpoint: serde_json::from_value(result.fields.get("privateEndpoint").cloned().unwrap_or_default())?
            ,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: result.fields.get("resourceGuid").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        tap_configurations: result.fields.get("tapConfigurations").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_machine: serde_json::from_value(result.fields.get("virtualMachine").cloned().unwrap_or_default())?
            ,
    })
}
