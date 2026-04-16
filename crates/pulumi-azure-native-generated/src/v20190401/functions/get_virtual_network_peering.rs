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
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The status of the virtual network peering.
    pub peering_state: Option<String>,
    /// The provisioning state of the resource.
    pub provisioning_state: Option<String>,
    /// The reference of the remote virtual network address space.
    pub remote_address_space: Option<network::v20190401::AddressSpaceResponse>,
    /// The reference of the remote virtual network. The remote virtual network can be in the same or different region (preview). See here to register for the preview and learn more (https://docs.microsoft.com/en-us/azure/virtual-network/virtual-network-create-peering).
    pub remote_virtual_network: Option<network::v20190401::SubResourceResponse>,
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
    let result = ctx.invoke("azure-native:network/v20190401:getVirtualNetworkPeering", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkPeeringResult {
        allow_forwarded_traffic: result.fields.get("allowForwardedTraffic").cloned().map(serde_json::from_value).transpose()?,
        allow_gateway_transit: result.fields.get("allowGatewayTransit").cloned().map(serde_json::from_value).transpose()?,
        allow_virtual_network_access: result.fields.get("allowVirtualNetworkAccess").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        peering_state: result.fields.get("peeringState").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        remote_address_space: result.fields.get("remoteAddressSpace").cloned().map(serde_json::from_value).transpose()?,
        remote_virtual_network: result.fields.get("remoteVirtualNetwork").cloned().map(serde_json::from_value).transpose()?,
        use_remote_gateways: result.fields.get("useRemoteGateways").cloned().map(serde_json::from_value).transpose()?,
    })
}
