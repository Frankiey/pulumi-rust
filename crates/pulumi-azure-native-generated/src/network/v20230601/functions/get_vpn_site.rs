use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a VPN site.
#[derive(Default)]
pub struct GetVpnSiteArgs {
    /// The resource group name of the VpnSite.
    pub resource_group_name: String,
    /// The name of the VpnSite being retrieved.
    pub vpn_site_name: String,
}

/// Result of the function invocation.
pub struct GetVpnSiteResult {
    /// The AddressSpace that contains an array of IP address ranges.
    pub address_space: Option<network::v20230601::AddressSpaceResponse>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The set of bgp properties.
    pub bgp_properties: Option<network::v20230601::BgpSettingsResponse>,
    /// The device properties.
    pub device_properties: Option<network::v20230601::DevicePropertiesResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The ip-address for the vpn-site.
    pub ip_address: Option<String>,
    /// IsSecuritySite flag.
    pub is_security_site: Option<bool>,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// Office365 Policy.
    pub o365policy: Option<network::v20230601::O365PolicyPropertiesResponse>,
    /// The provisioning state of the VPN site resource.
    pub provisioning_state: String,
    /// The key for vpn-site that can be used for connections.
    pub site_key: Option<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The VirtualWAN to which the vpnSite belongs.
    pub virtual_wan: Option<network::v20230601::SubResourceResponse>,
    /// List of all vpn site links.
    pub vpn_site_links: Option<Vec<network::v20230601::VpnSiteLinkResponse>>,
}

/// Retrieves the details of a VPN site.
pub async fn get_vpn_site(
    ctx: &Context,
    args: GetVpnSiteArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVpnSiteResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("vpnSiteName".to_string(), json!(args.vpn_site_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230601:getVpnSite", invoke_args, &opts).await?;

    Ok(GetVpnSiteResult {
        address_space: result.fields.get("addressSpace").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bgp_properties: result.fields.get("bgpProperties").cloned().map(serde_json::from_value).transpose()?,
        device_properties: result.fields.get("deviceProperties").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_address: result.fields.get("ipAddress").cloned().map(serde_json::from_value).transpose()?,
        is_security_site: result.fields.get("isSecuritySite").cloned().map(serde_json::from_value).transpose()?,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        o365policy: result.fields.get("o365Policy").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        site_key: result.fields.get("siteKey").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_wan: result.fields.get("virtualWan").cloned().map(serde_json::from_value).transpose()?,
        vpn_site_links: result.fields.get("vpnSiteLinks").cloned().map(serde_json::from_value).transpose()?,
    })
}
