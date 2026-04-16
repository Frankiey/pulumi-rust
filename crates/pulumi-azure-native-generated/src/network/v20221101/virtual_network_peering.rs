use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Peerings in a virtual network resource.
pub struct VirtualNetworkPeeringArgs {
    /// Whether the forwarded traffic from the VMs in the local virtual network will be allowed/disallowed in remote virtual network.
    pub allow_forwarded_traffic: Option<Input<bool>>,
    /// If gateway links can be used in remote virtual networking to link to this virtual network.
    pub allow_gateway_transit: Option<Input<bool>>,
    /// Whether the VMs in the local virtual network space would be able to access the VMs in remote virtual network space.
    pub allow_virtual_network_access: Option<Input<bool>>,
    /// If we need to verify the provisioning state of the remote gateway.
    pub do_not_verify_remote_gateways: Option<Input<bool>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The status of the virtual network peering.
    pub peering_state: Option<Input<serde_json::Value>>,
    /// The peering sync status of the virtual network peering.
    pub peering_sync_level: Option<Input<serde_json::Value>>,
    /// The reference to the address space peered with the remote virtual network.
    pub remote_address_space: Option<Input<network::v20221101::AddressSpaceArgs>>,
    /// The reference to the remote virtual network's Bgp Communities.
    pub remote_bgp_communities: Option<Input<network::v20221101::VirtualNetworkBgpCommunitiesArgs>>,
    /// The reference to the remote virtual network. The remote virtual network can be in the same or different region (preview). See here to register for the preview and learn more (https://docs.microsoft.com/en-us/azure/virtual-network/virtual-network-create-peering).
    pub remote_virtual_network: Option<Input<network::v20221101::SubResourceArgs>>,
    /// The reference to the current address space of the remote virtual network.
    pub remote_virtual_network_address_space: Option<Input<network::v20221101::AddressSpaceArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Parameter indicates the intention to sync the peering with the current address space on the remote vNet after it's updated.
    pub sync_remote_address_space: Option<Input<String>>,
    /// Resource type.
    pub type_: Option<Input<String>>,
    /// If remote gateways can be used on this virtual network. If the flag is set to true, and allowGatewayTransit on remote peering is also true, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to true. This flag cannot be set if virtual network already has a gateway.
    pub use_remote_gateways: Option<Input<bool>>,
    /// The name of the virtual network.
    pub virtual_network_name: Input<String>,
    /// The name of the peering.
    pub virtual_network_peering_name: Option<Input<String>>,
}

/// Peerings in a virtual network resource.
pub struct VirtualNetworkPeering {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Whether the forwarded traffic from the VMs in the local virtual network will be allowed/disallowed in remote virtual network.
    pub allow_forwarded_traffic: Output<serde_json::Value>,
    /// If gateway links can be used in remote virtual networking to link to this virtual network.
    pub allow_gateway_transit: Output<serde_json::Value>,
    /// Whether the VMs in the local virtual network space would be able to access the VMs in remote virtual network space.
    pub allow_virtual_network_access: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// If we need to verify the provisioning state of the remote gateway.
    pub do_not_verify_remote_gateways: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The status of the virtual network peering.
    pub peering_state: Output<serde_json::Value>,
    /// The peering sync status of the virtual network peering.
    pub peering_sync_level: Output<serde_json::Value>,
    /// The provisioning state of the virtual network peering resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The reference to the address space peered with the remote virtual network.
    pub remote_address_space: Output<serde_json::Value>,
    /// The reference to the remote virtual network's Bgp Communities.
    pub remote_bgp_communities: Output<serde_json::Value>,
    /// The reference to the remote virtual network. The remote virtual network can be in the same or different region (preview). See here to register for the preview and learn more (https://docs.microsoft.com/en-us/azure/virtual-network/virtual-network-create-peering).
    pub remote_virtual_network: Output<serde_json::Value>,
    /// The reference to the current address space of the remote virtual network.
    pub remote_virtual_network_address_space: Output<serde_json::Value>,
    /// The reference to the remote virtual network's encryption
    pub remote_virtual_network_encryption: Output<serde_json::Value>,
    /// The resourceGuid property of the Virtual Network peering resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// If remote gateways can be used on this virtual network. If the flag is set to true, and allowGatewayTransit on remote peering is also true, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to true. This flag cannot be set if virtual network already has a gateway.
    pub use_remote_gateways: Output<serde_json::Value>,
}

impl VirtualNetworkPeering {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20221101:VirtualNetworkPeering";

    /// Create a new VirtualNetworkPeering resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualNetworkPeeringArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.allow_forwarded_traffic {
            pulumi_sdk::resolve_input("allowForwardedTraffic", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_gateway_transit {
            pulumi_sdk::resolve_input("allowGatewayTransit", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_virtual_network_access {
            pulumi_sdk::resolve_input("allowVirtualNetworkAccess", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.do_not_verify_remote_gateways {
            pulumi_sdk::resolve_input("doNotVerifyRemoteGateways", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peering_state {
            pulumi_sdk::resolve_input("peeringState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peering_sync_level {
            pulumi_sdk::resolve_input("peeringSyncLevel", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.remote_address_space {
            pulumi_sdk::resolve_input("remoteAddressSpace", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.remote_bgp_communities {
            pulumi_sdk::resolve_input("remoteBgpCommunities", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.remote_virtual_network {
            pulumi_sdk::resolve_input("remoteVirtualNetwork", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.remote_virtual_network_address_space {
            pulumi_sdk::resolve_input("remoteVirtualNetworkAddressSpace", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.sync_remote_address_space {
            pulumi_sdk::resolve_input("syncRemoteAddressSpace", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.type_ {
            pulumi_sdk::resolve_input("type", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.use_remote_gateways {
            pulumi_sdk::resolve_input("useRemoteGateways", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualNetworkName", args.virtual_network_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.virtual_network_peering_name {
            pulumi_sdk::resolve_input("virtualNetworkPeeringName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            allow_forwarded_traffic: registered.outputs.get("allowForwardedTraffic")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_gateway_transit: registered.outputs.get("allowGatewayTransit")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_virtual_network_access: registered.outputs.get("allowVirtualNetworkAccess")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            do_not_verify_remote_gateways: registered.outputs.get("doNotVerifyRemoteGateways")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peering_state: registered.outputs.get("peeringState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peering_sync_level: registered.outputs.get("peeringSyncLevel")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_address_space: registered.outputs.get("remoteAddressSpace")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_bgp_communities: registered.outputs.get("remoteBgpCommunities")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_virtual_network: registered.outputs.get("remoteVirtualNetwork")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_virtual_network_address_space: registered.outputs.get("remoteVirtualNetworkAddressSpace")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_virtual_network_encryption: registered.outputs.get("remoteVirtualNetworkEncryption")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            use_remote_gateways: registered.outputs.get("useRemoteGateways")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
