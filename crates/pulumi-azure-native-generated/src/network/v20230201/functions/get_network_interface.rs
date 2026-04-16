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
    /// Auxiliary mode of Network Interface resource.
    pub auxiliary_mode: Option<String>,
    /// Auxiliary sku of Network Interface resource.
    pub auxiliary_sku: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Indicates whether to disable tcp state tracking.
    pub disable_tcp_state_tracking: Option<bool>,
    /// The DNS settings in network interface.
    pub dns_settings: Option<network::v20230201::NetworkInterfaceDnsSettingsResponse>,
    /// A reference to the dscp configuration to which the network interface is linked.
    pub dscp_configuration: network::v20230201::SubResourceResponse,
    /// If the network interface is configured for accelerated networking. Not applicable to VM sizes which require accelerated networking.
    pub enable_accelerated_networking: Option<bool>,
    /// Indicates whether IP forwarding is enabled on this network interface.
    pub enable_ip_forwarding: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The extended location of the network interface.
    pub extended_location: Option<network::v20230201::ExtendedLocationResponse>,
    /// A list of references to linked BareMetal resources.
    pub hosted_workloads: Vec<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// A list of IPConfigurations of the network interface.
    pub ip_configurations: Option<Vec<network::v20230201::NetworkInterfaceIPConfigurationResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// The MAC address of the network interface.
    pub mac_address: String,
    /// Migration phase of Network Interface resource.
    pub migration_phase: Option<String>,
    /// Resource name.
    pub name: String,
    /// The reference to the NetworkSecurityGroup resource.
    pub network_security_group: Option<network::v20230201::NetworkSecurityGroupResponse>,
    /// Type of Network Interface resource.
    pub nic_type: Option<String>,
    /// Whether this is a primary network interface on a virtual machine.
    pub primary: bool,
    /// A reference to the private endpoint to which the network interface is linked.
    pub private_endpoint: network::v20230201::PrivateEndpointResponse,
    /// Privatelinkservice of the network interface resource.
    pub private_link_service: Option<network::v20230201::PrivateLinkServiceResponse>,
    /// The provisioning state of the network interface resource.
    pub provisioning_state: String,
    /// The resource GUID property of the network interface resource.
    pub resource_guid: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// A list of TapConfigurations of the network interface.
    pub tap_configurations: Vec<network::v20230201::NetworkInterfaceTapConfigurationResponse>,
    /// Resource type.
    pub type_: String,
    /// The reference to a virtual machine.
    pub virtual_machine: network::v20230201::SubResourceResponse,
    /// Whether the virtual machine this nic is attached to supports encryption.
    pub vnet_encryption_supported: bool,
    /// WorkloadType of the NetworkInterface for BareMetal resources
    pub workload_type: Option<String>,
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
    let result = ctx.invoke("azure-native:network/v20230201:getNetworkInterface", invoke_args, &opts).await?;

    Ok(GetNetworkInterfaceResult {
        auxiliary_mode: result.fields.get("auxiliaryMode").cloned().map(serde_json::from_value).transpose()?,
        auxiliary_sku: result.fields.get("auxiliarySku").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        disable_tcp_state_tracking: result.fields.get("disableTcpStateTracking").cloned().map(serde_json::from_value).transpose()?,
        dns_settings: result.fields.get("dnsSettings").cloned().map(serde_json::from_value).transpose()?,
        dscp_configuration: serde_json::from_value(result.fields.get("dscpConfiguration").cloned().unwrap_or_default())?
            ,
        enable_accelerated_networking: result.fields.get("enableAcceleratedNetworking").cloned().map(serde_json::from_value).transpose()?,
        enable_ip_forwarding: result.fields.get("enableIPForwarding").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        extended_location: result.fields.get("extendedLocation").cloned().map(serde_json::from_value).transpose()?,
        hosted_workloads: serde_json::from_value(result.fields.get("hostedWorkloads").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        mac_address: serde_json::from_value(result.fields.get("macAddress").cloned().unwrap_or_default())?
            ,
        migration_phase: result.fields.get("migrationPhase").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_security_group: result.fields.get("networkSecurityGroup").cloned().map(serde_json::from_value).transpose()?,
        nic_type: result.fields.get("nicType").cloned().map(serde_json::from_value).transpose()?,
        primary: serde_json::from_value(result.fields.get("primary").cloned().unwrap_or_default())?
            ,
        private_endpoint: serde_json::from_value(result.fields.get("privateEndpoint").cloned().unwrap_or_default())?
            ,
        private_link_service: result.fields.get("privateLinkService").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        tap_configurations: serde_json::from_value(result.fields.get("tapConfigurations").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_machine: serde_json::from_value(result.fields.get("virtualMachine").cloned().unwrap_or_default())?
            ,
        vnet_encryption_supported: serde_json::from_value(result.fields.get("vnetEncryptionSupported").cloned().unwrap_or_default())?
            ,
        workload_type: result.fields.get("workloadType").cloned().map(serde_json::from_value).transpose()?,
    })
}
