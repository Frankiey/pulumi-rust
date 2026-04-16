use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Virtual Network resource.
pub struct VirtualNetworkArgs {
    /// The AddressSpace that contains an array of IP address ranges that can be used by subnets.
    pub address_space: Option<Input<network::v20220701::AddressSpaceArgs>>,
    /// Bgp Communities sent over ExpressRoute with each route corresponding to a prefix in this VNET.
    pub bgp_communities: Option<Input<network::v20220701::VirtualNetworkBgpCommunitiesArgs>>,
    /// The DDoS protection plan associated with the virtual network.
    pub ddos_protection_plan: Option<Input<network::v20220701::SubResourceArgs>>,
    /// The dhcpOptions that contains an array of DNS servers available to VMs deployed in the virtual network.
    pub dhcp_options: Option<Input<network::v20220701::DhcpOptionsArgs>>,
    /// Indicates if DDoS protection is enabled for all the protected resources in the virtual network. It requires a DDoS protection plan associated with the resource.
    pub enable_ddos_protection: Option<Input<bool>>,
    /// Indicates if VM protection is enabled for all the subnets in the virtual network.
    pub enable_vm_protection: Option<Input<bool>>,
    /// Indicates if encryption is enabled on virtual network and if VM without encryption is allowed in encrypted VNet.
    pub encryption: Option<Input<network::v20220701::VirtualNetworkEncryptionArgs>>,
    /// The extended location of the virtual network.
    pub extended_location: Option<Input<network::v20220701::ExtendedLocationArgs>>,
    /// The FlowTimeout value (in minutes) for the Virtual Network
    pub flow_timeout_in_minutes: Option<Input<i64>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Array of IpAllocation which reference this VNET.
    pub ip_allocations: Option<Vec<Input<network::v20220701::SubResourceArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// A list of subnets in a Virtual Network.
    pub subnets: Option<Vec<Input<network::v20220701::SubnetArgs>>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The name of the virtual network.
    pub virtual_network_name: Option<Input<String>>,
    /// A list of peerings in a Virtual Network.
    pub virtual_network_peerings: Option<Vec<Input<network::v20220701::VirtualNetworkPeeringArgs>>>,
}

/// Virtual Network resource.
pub struct VirtualNetwork {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The AddressSpace that contains an array of IP address ranges that can be used by subnets.
    pub address_space: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Bgp Communities sent over ExpressRoute with each route corresponding to a prefix in this VNET.
    pub bgp_communities: Output<serde_json::Value>,
    /// The DDoS protection plan associated with the virtual network.
    pub ddos_protection_plan: Output<serde_json::Value>,
    /// The dhcpOptions that contains an array of DNS servers available to VMs deployed in the virtual network.
    pub dhcp_options: Output<serde_json::Value>,
    /// Indicates if DDoS protection is enabled for all the protected resources in the virtual network. It requires a DDoS protection plan associated with the resource.
    pub enable_ddos_protection: Output<serde_json::Value>,
    /// Indicates if VM protection is enabled for all the subnets in the virtual network.
    pub enable_vm_protection: Output<serde_json::Value>,
    /// Indicates if encryption is enabled on virtual network and if VM without encryption is allowed in encrypted VNet.
    pub encryption: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the virtual network.
    pub extended_location: Output<serde_json::Value>,
    /// The FlowTimeout value (in minutes) for the Virtual Network
    pub flow_timeout_in_minutes: Output<serde_json::Value>,
    /// Array of IpAllocation which reference this VNET.
    pub ip_allocations: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the virtual network resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resourceGuid property of the Virtual Network resource.
    pub resource_guid: Output<serde_json::Value>,
    /// A list of subnets in a Virtual Network.
    pub subnets: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// A list of peerings in a Virtual Network.
    pub virtual_network_peerings: Output<serde_json::Value>,
}

impl VirtualNetwork {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220701:VirtualNetwork";

    /// Create a new VirtualNetwork resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualNetworkArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.address_space {
            pulumi_sdk::resolve_input("addressSpace", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.bgp_communities {
            pulumi_sdk::resolve_input("bgpCommunities", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ddos_protection_plan {
            pulumi_sdk::resolve_input("ddosProtectionPlan", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.dhcp_options {
            pulumi_sdk::resolve_input("dhcpOptions", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_ddos_protection {
            pulumi_sdk::resolve_input("enableDdosProtection", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_vm_protection {
            pulumi_sdk::resolve_input("enableVmProtection", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.encryption {
            pulumi_sdk::resolve_input("encryption", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.flow_timeout_in_minutes {
            pulumi_sdk::resolve_input("flowTimeoutInMinutes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_allocations {
            pulumi_sdk::resolve_input_vec("ipAllocations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.subnets {
            pulumi_sdk::resolve_input_vec("subnets", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network_name {
            pulumi_sdk::resolve_input("virtualNetworkName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network_peerings {
            pulumi_sdk::resolve_input_vec("virtualNetworkPeerings", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            address_space: registered.outputs.get("addressSpace")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            bgp_communities: registered.outputs.get("bgpCommunities")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ddos_protection_plan: registered.outputs.get("ddosProtectionPlan")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            dhcp_options: registered.outputs.get("dhcpOptions")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_ddos_protection: registered.outputs.get("enableDdosProtection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_vm_protection: registered.outputs.get("enableVmProtection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            encryption: registered.outputs.get("encryption")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            flow_timeout_in_minutes: registered.outputs.get("flowTimeoutInMinutes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_allocations: registered.outputs.get("ipAllocations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            subnets: registered.outputs.get("subnets")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network_peerings: registered.outputs.get("virtualNetworkPeerings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
