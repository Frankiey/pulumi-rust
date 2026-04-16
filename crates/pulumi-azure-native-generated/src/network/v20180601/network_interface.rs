use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// A network interface in a resource group.
pub struct NetworkInterfaceArgs {
    /// The DNS settings in network interface.
    pub dns_settings: Option<Input<network::v20180601::NetworkInterfaceDnsSettingsArgs>>,
    /// If the network interface is accelerated networking enabled.
    pub enable_accelerated_networking: Option<Input<bool>>,
    /// Indicates whether IP forwarding is enabled on this network interface.
    pub enable_ip_forwarding: Option<Input<bool>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// A list of IPConfigurations of the network interface.
    pub ip_configurations: Option<Vec<Input<network::v20180601::NetworkInterfaceIPConfigurationArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The MAC address of the network interface.
    pub mac_address: Option<Input<String>>,
    /// The name of the network interface.
    pub network_interface_name: Option<Input<String>>,
    /// The reference of the NetworkSecurityGroup resource.
    pub network_security_group: Option<Input<network::v20180601::NetworkSecurityGroupArgs>>,
    /// Gets whether this is a primary network interface on a virtual machine.
    pub primary: Option<Input<bool>>,
    /// The provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The resource GUID property of the network interface resource.
    pub resource_guid: Option<Input<String>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The reference of a virtual machine.
    pub virtual_machine: Option<Input<network::v20180601::SubResourceArgs>>,
}

/// A network interface in a resource group.
pub struct NetworkInterface {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The DNS settings in network interface.
    pub dns_settings: Output<serde_json::Value>,
    /// If the network interface is accelerated networking enabled.
    pub enable_accelerated_networking: Output<serde_json::Value>,
    /// Indicates whether IP forwarding is enabled on this network interface.
    pub enable_ip_forwarding: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// A list of IPConfigurations of the network interface.
    pub ip_configurations: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// The MAC address of the network interface.
    pub mac_address: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The reference of the NetworkSecurityGroup resource.
    pub network_security_group: Output<serde_json::Value>,
    /// Gets whether this is a primary network interface on a virtual machine.
    pub primary: Output<serde_json::Value>,
    /// The provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the network interface resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The reference of a virtual machine.
    pub virtual_machine: Output<serde_json::Value>,
}

impl NetworkInterface {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20180601:NetworkInterface";

    /// Create a new NetworkInterface resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkInterfaceArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.dns_settings {
            pulumi_sdk::resolve_input("dnsSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_accelerated_networking {
            pulumi_sdk::resolve_input("enableAcceleratedNetworking", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_ip_forwarding {
            pulumi_sdk::resolve_input("enableIPForwarding", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_configurations {
            pulumi_sdk::resolve_input_vec("ipConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.mac_address {
            pulumi_sdk::resolve_input("macAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_interface_name {
            pulumi_sdk::resolve_input("networkInterfaceName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_security_group {
            pulumi_sdk::resolve_input("networkSecurityGroup", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.primary {
            pulumi_sdk::resolve_input("primary", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.provisioning_state {
            pulumi_sdk::resolve_input("provisioningState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.resource_guid {
            pulumi_sdk::resolve_input("resourceGuid", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_machine {
            pulumi_sdk::resolve_input("virtualMachine", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dns_settings: registered.outputs.get("dnsSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_accelerated_networking: registered.outputs.get("enableAcceleratedNetworking")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_ip_forwarding: registered.outputs.get("enableIPForwarding")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            mac_address: registered.outputs.get("macAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_security_group: registered.outputs.get("networkSecurityGroup")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            primary: registered.outputs.get("primary")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_machine: registered.outputs.get("virtualMachine")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
