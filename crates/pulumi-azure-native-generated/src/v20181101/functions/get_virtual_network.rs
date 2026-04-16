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
    pub address_space: Option<network::v20181101::AddressSpaceResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The DDoS protection plan associated with the virtual network.
    pub ddos_protection_plan: Option<network::v20181101::SubResourceResponse>,
    /// The dhcpOptions that contains an array of DNS servers available to VMs deployed in the virtual network.
    pub dhcp_options: Option<network::v20181101::DhcpOptionsResponse>,
    /// Indicates if DDoS protection is enabled for all the protected resources in the virtual network. It requires a DDoS protection plan associated with the resource.
    pub enable_ddos_protection: Option<bool>,
    /// Indicates if VM protection is enabled for all the subnets in the virtual network.
    pub enable_vm_protection: Option<bool>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the PublicIP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// The resourceGuid property of the Virtual Network resource.
    pub resource_guid: Option<String>,
    /// A list of subnets in a Virtual Network.
    pub subnets: Option<Vec<network::v20181101::SubnetResponse>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// A list of peerings in a Virtual Network.
    pub virtual_network_peerings: Option<Vec<network::v20181101::VirtualNetworkPeeringResponse>>,
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
    let result = ctx.invoke("azure-native:network/v20181101:getVirtualNetwork", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkResult {
        address_space: result.fields.get("addressSpace").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        ddos_protection_plan: result.fields.get("ddosProtectionPlan").cloned().map(serde_json::from_value).transpose()?,
        dhcp_options: result.fields.get("dhcpOptions").cloned().map(serde_json::from_value).transpose()?,
        enable_ddos_protection: result.fields.get("enableDdosProtection").cloned().map(serde_json::from_value).transpose()?,
        enable_vm_protection: result.fields.get("enableVmProtection").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: result.fields.get("resourceGuid").cloned().map(serde_json::from_value).transpose()?,
        subnets: result.fields.get("subnets").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_network_peerings: result.fields.get("virtualNetworkPeerings").cloned().map(serde_json::from_value).transpose()?,
    })
}
