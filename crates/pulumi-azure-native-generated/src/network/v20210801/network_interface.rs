use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// A network interface in a resource group.
pub struct NetworkInterfaceArgs {
    /// Auxiliary mode of Network Interface resource.
    pub auxiliary_mode: Option<Input<serde_json::Value>>,
    /// The DNS settings in network interface.
    pub dns_settings: Option<Input<network::v20210801::NetworkInterfaceDnsSettingsArgs>>,
    /// If the network interface is accelerated networking enabled.
    pub enable_accelerated_networking: Option<Input<bool>>,
    /// Indicates whether IP forwarding is enabled on this network interface.
    pub enable_ip_forwarding: Option<Input<bool>>,
    /// The extended location of the network interface.
    pub extended_location: Option<Input<network::v20210801::ExtendedLocationArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// A list of IPConfigurations of the network interface.
    pub ip_configurations: Option<Vec<Input<network::v20210801::NetworkInterfaceIPConfigurationArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// Migration phase of Network Interface resource.
    pub migration_phase: Option<Input<serde_json::Value>>,
    /// The name of the network interface.
    pub network_interface_name: Option<Input<String>>,
    /// The reference to the NetworkSecurityGroup resource.
    pub network_security_group: Option<Input<network::v20210801::NetworkSecurityGroupArgs>>,
    /// Type of Network Interface resource.
    pub nic_type: Option<Input<serde_json::Value>>,
    /// Privatelinkservice of the network interface resource.
    pub private_link_service: Option<Input<network::v20210801::PrivateLinkServiceArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// WorkloadType of the NetworkInterface for BareMetal resources
    pub workload_type: Option<Input<String>>,
}

/// A network interface in a resource group.
pub struct NetworkInterface {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Auxiliary mode of Network Interface resource.
    pub auxiliary_mode: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The DNS settings in network interface.
    pub dns_settings: Output<serde_json::Value>,
    /// A reference to the dscp configuration to which the network interface is linked.
    pub dscp_configuration: Output<serde_json::Value>,
    /// If the network interface is accelerated networking enabled.
    pub enable_accelerated_networking: Output<serde_json::Value>,
    /// Indicates whether IP forwarding is enabled on this network interface.
    pub enable_ip_forwarding: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the network interface.
    pub extended_location: Output<serde_json::Value>,
    /// A list of references to linked BareMetal resources.
    pub hosted_workloads: Output<serde_json::Value>,
    /// A list of IPConfigurations of the network interface.
    pub ip_configurations: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// The MAC address of the network interface.
    pub mac_address: Output<serde_json::Value>,
    /// Migration phase of Network Interface resource.
    pub migration_phase: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The reference to the NetworkSecurityGroup resource.
    pub network_security_group: Output<serde_json::Value>,
    /// Type of Network Interface resource.
    pub nic_type: Output<serde_json::Value>,
    /// Whether this is a primary network interface on a virtual machine.
    pub primary: Output<serde_json::Value>,
    /// A reference to the private endpoint to which the network interface is linked.
    pub private_endpoint: Output<serde_json::Value>,
    /// Privatelinkservice of the network interface resource.
    pub private_link_service: Output<serde_json::Value>,
    /// The provisioning state of the network interface resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the network interface resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// A list of TapConfigurations of the network interface.
    pub tap_configurations: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The reference to a virtual machine.
    pub virtual_machine: Output<serde_json::Value>,
    /// Whether the virtual machine this nic is attached to supports encryption.
    pub vnet_encryption_supported: Output<serde_json::Value>,
    /// WorkloadType of the NetworkInterface for BareMetal resources
    pub workload_type: Output<serde_json::Value>,
}

impl NetworkInterface {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20210801:NetworkInterface";

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

        if let Some(v) = args.auxiliary_mode {
            pulumi_sdk::resolve_input("auxiliaryMode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dns_settings {
            pulumi_sdk::resolve_input("dnsSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_accelerated_networking {
            pulumi_sdk::resolve_input("enableAcceleratedNetworking", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_ip_forwarding {
            pulumi_sdk::resolve_input("enableIPForwarding", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
        if let Some(v) = args.migration_phase {
            pulumi_sdk::resolve_input("migrationPhase", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_interface_name {
            pulumi_sdk::resolve_input("networkInterfaceName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_security_group {
            pulumi_sdk::resolve_input("networkSecurityGroup", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nic_type {
            pulumi_sdk::resolve_input("nicType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_link_service {
            pulumi_sdk::resolve_input("privateLinkService", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.workload_type {
            pulumi_sdk::resolve_input("workloadType", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            auxiliary_mode: registered.outputs.get("auxiliaryMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dns_settings: registered.outputs.get("dnsSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dscp_configuration: registered.outputs.get("dscpConfiguration")
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
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            hosted_workloads: registered.outputs.get("hostedWorkloads")
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
            migration_phase: registered.outputs.get("migrationPhase")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_security_group: registered.outputs.get("networkSecurityGroup")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            nic_type: registered.outputs.get("nicType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            primary: registered.outputs.get("primary")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_endpoint: registered.outputs.get("privateEndpoint")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_link_service: registered.outputs.get("privateLinkService")
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
            tap_configurations: registered.outputs.get("tapConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_machine: registered.outputs.get("virtualMachine")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vnet_encryption_supported: registered.outputs.get("vnetEncryptionSupported")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            workload_type: registered.outputs.get("workloadType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
