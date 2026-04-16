use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified virtual network by resource group.
#[derive(Default)]
pub struct GetVirtualNetworkArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network.
    pub virtual_network_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkResult {
    /// The AddressSpace that contains an array of IP address ranges that can be used by subnets.
    pub address_space: Option<network::v20250301::AddressSpaceResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Bgp Communities sent over ExpressRoute with each route corresponding to a prefix in this VNET.
    pub bgp_communities: Option<network::v20250301::VirtualNetworkBgpCommunitiesResponse>,
    /// The DDoS protection plan associated with the virtual network.
    pub ddos_protection_plan: Option<network::v20250301::SubResourceResponse>,
    /// A reference to the default public nat gateway being used by this virtual network resource.
    pub default_public_nat_gateway: network::v20250301::SubResourceResponse,
    /// The dhcpOptions that contains an array of DNS servers available to VMs deployed in the virtual network.
    pub dhcp_options: Option<network::v20250301::DhcpOptionsResponse>,
    /// Indicates if DDoS protection is enabled for all the protected resources in the virtual network. It requires a DDoS protection plan associated with the resource.
    pub enable_ddos_protection: Option<bool>,
    /// Indicates if VM protection is enabled for all the subnets in the virtual network.
    pub enable_vm_protection: Option<bool>,
    /// Indicates if encryption is enabled on virtual network and if VM without encryption is allowed in encrypted VNet.
    pub encryption: Option<network::v20250301::VirtualNetworkEncryptionResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The extended location of the virtual network.
    pub extended_location: Option<network::v20250301::ExtendedLocationResponse>,
    /// A collection of references to flow log resources.
    pub flow_logs: Vec<network::v20250301::FlowLogResponse>,
    /// The FlowTimeout value (in minutes) for the Virtual Network
    pub flow_timeout_in_minutes: Option<i64>,
    /// Resource ID.
    pub id: Option<String>,
    /// Array of IpAllocation which reference this VNET.
    pub ip_allocations: Option<Vec<network::v20250301::SubResourceResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Private Endpoint VNet Policies.
    pub private_endpoint_v_net_policies: Option<String>,
    /// The provisioning state of the virtual network resource.
    pub provisioning_state: String,
    /// The resourceGuid property of the Virtual Network resource.
    pub resource_guid: String,
    /// A list of subnets in a Virtual Network.
    pub subnets: Option<Vec<network::v20250301::SubnetResponse>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// A list of peerings in a Virtual Network.
    pub virtual_network_peerings: Option<Vec<network::v20250301::VirtualNetworkPeeringResponse>>,
}

/// Gets the specified virtual network by resource group.
pub async fn get_virtual_network(
    ctx: &Context,
    args: GetVirtualNetworkArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkName".to_string(), json!(args.virtual_network_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getVirtualNetwork", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkResult {
        address_space: result.fields.get("addressSpace").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bgp_communities: result.fields.get("bgpCommunities").cloned().map(serde_json::from_value).transpose()?,
        ddos_protection_plan: result.fields.get("ddosProtectionPlan").cloned().map(serde_json::from_value).transpose()?,
        default_public_nat_gateway: serde_json::from_value(result.fields.get("defaultPublicNatGateway").cloned().unwrap_or_default())?
            ,
        dhcp_options: result.fields.get("dhcpOptions").cloned().map(serde_json::from_value).transpose()?,
        enable_ddos_protection: result.fields.get("enableDdosProtection").cloned().map(serde_json::from_value).transpose()?,
        enable_vm_protection: result.fields.get("enableVmProtection").cloned().map(serde_json::from_value).transpose()?,
        encryption: result.fields.get("encryption").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        extended_location: result.fields.get("extendedLocation").cloned().map(serde_json::from_value).transpose()?,
        flow_logs: serde_json::from_value(result.fields.get("flowLogs").cloned().unwrap_or_default())?
            ,
        flow_timeout_in_minutes: result.fields.get("flowTimeoutInMinutes").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_allocations: result.fields.get("ipAllocations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        private_endpoint_v_net_policies: result.fields.get("privateEndpointVNetPolicies").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        subnets: result.fields.get("subnets").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_network_peerings: result.fields.get("virtualNetworkPeerings").cloned().map(serde_json::from_value).transpose()?,
    })
}
