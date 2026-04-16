use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a VirtualWAN.
#[derive(Default)]
pub struct GetVirtualWANArgs {
    /// The resource group name of the VirtualWan.
    pub resource_group_name: String,
    /// The name of the VirtualWAN being retrieved.
    pub virtual_wan_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualWANResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Vpn encryption to be disabled or not.
    pub disable_vpn_encryption: Option<bool>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// List of VirtualHubs in the VirtualWAN.
    pub virtual_hubs: Vec<network::v20180701::SubResourceResponse>,
    pub vpn_sites: Vec<network::v20180701::SubResourceResponse>,
}

/// Retrieves the details of a VirtualWAN.
pub async fn get_virtual_wan(
    ctx: &Context,
    args: GetVirtualWANArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualWANResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualWANName".to_string(), json!(args.virtual_wan_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20180701:getVirtualWAN", invoke_args, &opts).await?;

    Ok(GetVirtualWANResult {
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
