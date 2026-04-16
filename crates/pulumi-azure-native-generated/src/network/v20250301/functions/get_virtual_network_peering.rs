use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified virtual network peering.
#[derive(Default)]
pub struct GetVirtualNetworkPeeringArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network.
    pub virtual_network_name: String,
    /// The name of the virtual network peering.
    pub virtual_network_peering_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkPeeringResult {
    /// Whether the forwarded traffic from the VMs in the local virtual network will be allowed/disallowed in remote virtual network.
    pub allow_forwarded_traffic: Option<bool>,
    /// If gateway links can be used in remote virtual networking to link to this virtual network.
    pub allow_gateway_transit: Option<bool>,
    /// Whether the VMs in the local virtual network space would be able to access the VMs in remote virtual network space.
    pub allow_virtual_network_access: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// If we need to verify the provisioning state of the remote gateway.
    pub do_not_verify_remote_gateways: Option<bool>,
    /// Whether only Ipv6 address space is peered for subnet peering.
    pub enable_only_i_pv6peering: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The local address space of the local virtual network that is peered.
    pub local_address_space: Option<network::v20250301::AddressSpaceResponse>,
    /// List of local subnet names that are subnet peered with remote virtual network.
    pub local_subnet_names: Option<Vec<String>>,
    /// The current local address space of the local virtual network that is peered.
    pub local_virtual_network_address_space: Option<network::v20250301::AddressSpaceResponse>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Whether complete virtual network address space is peered.
    pub peer_complete_vnets: Option<bool>,
    /// The status of the virtual network peering.
    pub peering_state: Option<String>,
    /// The peering sync status of the virtual network peering.
    pub peering_sync_level: Option<String>,
    /// The provisioning state of the virtual network peering resource.
    pub provisioning_state: String,
    /// The reference to the address space peered with the remote virtual network.
    pub remote_address_space: Option<network::v20250301::AddressSpaceResponse>,
    /// The reference to the remote virtual network's Bgp Communities.
    pub remote_bgp_communities: Option<network::v20250301::VirtualNetworkBgpCommunitiesResponse>,
    /// List of remote subnet names from remote virtual network that are subnet peered.
    pub remote_subnet_names: Option<Vec<String>>,
    /// The reference to the remote virtual network. The remote virtual network can be in the same or different region (preview). See here to register for the preview and learn more (https://docs.microsoft.com/en-us/azure/virtual-network/virtual-network-create-peering).
    pub remote_virtual_network: Option<network::v20250301::SubResourceResponse>,
    /// The reference to the current address space of the remote virtual network.
    pub remote_virtual_network_address_space: Option<network::v20250301::AddressSpaceResponse>,
    /// The reference to the remote virtual network's encryption
    pub remote_virtual_network_encryption: network::v20250301::VirtualNetworkEncryptionResponse,
    /// The resourceGuid property of the Virtual Network peering resource.
    pub resource_guid: String,
    /// Resource type.
    pub type_: Option<String>,
    /// If remote gateways can be used on this virtual network. If the flag is set to true, and allowGatewayTransit on remote peering is also true, virtual network will use gateways of remote virtual network for transit. Only one peering can have this flag set to true. This flag cannot be set if virtual network already has a gateway.
    pub use_remote_gateways: Option<bool>,
}

/// Gets the specified virtual network peering.
pub async fn get_virtual_network_peering(
    ctx: &Context,
    args: GetVirtualNetworkPeeringArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkPeeringResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkName".to_string(), json!(args.virtual_network_name));
    invoke_args.insert("virtualNetworkPeeringName".to_string(), json!(args.virtual_network_peering_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getVirtualNetworkPeering", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkPeeringResult {
        allow_forwarded_traffic: result.fields.get("allowForwardedTraffic").cloned().map(serde_json::from_value).transpose()?,
        allow_gateway_transit: result.fields.get("allowGatewayTransit").cloned().map(serde_json::from_value).transpose()?,
        allow_virtual_network_access: result.fields.get("allowVirtualNetworkAccess").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        do_not_verify_remote_gateways: result.fields.get("doNotVerifyRemoteGateways").cloned().map(serde_json::from_value).transpose()?,
        enable_only_i_pv6peering: result.fields.get("enableOnlyIPv6Peering").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        local_address_space: result.fields.get("localAddressSpace").cloned().map(serde_json::from_value).transpose()?,
        local_subnet_names: result.fields.get("localSubnetNames").cloned().map(serde_json::from_value).transpose()?,
        local_virtual_network_address_space: result.fields.get("localVirtualNetworkAddressSpace").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        peer_complete_vnets: result.fields.get("peerCompleteVnets").cloned().map(serde_json::from_value).transpose()?,
        peering_state: result.fields.get("peeringState").cloned().map(serde_json::from_value).transpose()?,
        peering_sync_level: result.fields.get("peeringSyncLevel").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        remote_address_space: result.fields.get("remoteAddressSpace").cloned().map(serde_json::from_value).transpose()?,
        remote_bgp_communities: result.fields.get("remoteBgpCommunities").cloned().map(serde_json::from_value).transpose()?,
        remote_subnet_names: result.fields.get("remoteSubnetNames").cloned().map(serde_json::from_value).transpose()?,
        remote_virtual_network: result.fields.get("remoteVirtualNetwork").cloned().map(serde_json::from_value).transpose()?,
        remote_virtual_network_address_space: result.fields.get("remoteVirtualNetworkAddressSpace").cloned().map(serde_json::from_value).transpose()?,
        remote_virtual_network_encryption: serde_json::from_value(result.fields.get("remoteVirtualNetworkEncryption").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        type_: result.fields.get("type").cloned().map(serde_json::from_value).transpose()?,
        use_remote_gateways: result.fields.get("useRemoteGateways").cloned().map(serde_json::from_value).transpose()?,
    })
}
