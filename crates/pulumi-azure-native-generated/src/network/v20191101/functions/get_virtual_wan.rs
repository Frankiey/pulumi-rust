use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a VirtualWAN.
#[derive(Default)]
pub struct GetVirtualWanArgs {
    /// The resource group name of the VirtualWan.
    pub resource_group_name: String,
    /// The name of the VirtualWAN being retrieved.
    pub virtual_wan_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualWanResult {
    /// True if branch to branch traffic is allowed.
    pub allow_branch_to_branch_traffic: Option<bool>,
    /// True if Vnet to Vnet traffic is allowed.
    pub allow_vnet_to_vnet_traffic: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Vpn encryption to be disabled or not.
    pub disable_vpn_encryption: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// The office local breakout category.
    pub office365local_breakout_category: String,
    /// The provisioning state of the virtual WAN resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// List of VirtualHubs in the VirtualWAN.
    pub virtual_hubs: Vec<network::v20191101::SubResourceResponse>,
    /// List of VpnSites in the VirtualWAN.
    pub vpn_sites: Vec<network::v20191101::SubResourceResponse>,
}

/// Retrieves the details of a VirtualWAN.
pub async fn get_virtual_wan(
    ctx: &Context,
    args: GetVirtualWanArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualWanResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualWANName".to_string(), json!(args.virtual_wan_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20191101:getVirtualWan", invoke_args, &opts).await?;

    Ok(GetVirtualWanResult {
        allow_branch_to_branch_traffic: result.fields.get("allowBranchToBranchTraffic").cloned().map(serde_json::from_value).transpose()?,
        allow_vnet_to_vnet_traffic: result.fields.get("allowVnetToVnetTraffic").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        disable_vpn_encryption: result.fields.get("disableVpnEncryption").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        office365local_breakout_category: serde_json::from_value(result.fields.get("office365LocalBreakoutCategory").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hubs: serde_json::from_value(result.fields.get("virtualHubs").cloned().unwrap_or_default())?
            ,
        vpn_sites: serde_json::from_value(result.fields.get("vpnSites").cloned().unwrap_or_default())?
            ,
    })
}
