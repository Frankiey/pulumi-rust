use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VpnSite Resource.
pub struct VpnSiteArgs {
    /// The AddressSpace that contains an array of IP address ranges.
    pub address_space: Option<Input<network::v20200501::AddressSpaceArgs>>,
    /// The set of bgp properties.
    pub bgp_properties: Option<Input<network::v20200501::BgpSettingsArgs>>,
    /// The device properties.
    pub device_properties: Option<Input<network::v20200501::DevicePropertiesArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The ip-address for the vpn-site.
    pub ip_address: Option<Input<String>>,
    /// IsSecuritySite flag.
    pub is_security_site: Option<Input<bool>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The resource group name of the VpnSite.
    pub resource_group_name: Input<String>,
    /// The key for vpn-site that can be used for connections.
    pub site_key: Option<Input<String>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The VirtualWAN to which the vpnSite belongs.
    pub virtual_wan: Option<Input<network::v20200501::SubResourceArgs>>,
    /// List of all vpn site links.
    pub vpn_site_links: Option<Vec<Input<network::v20200501::VpnSiteLinkArgs>>>,
    /// The name of the VpnSite being created or updated.
    pub vpn_site_name: Option<Input<String>>,
}

/// VpnSite Resource.
pub struct VpnSite {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The AddressSpace that contains an array of IP address ranges.
    pub address_space: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The set of bgp properties.
    pub bgp_properties: Output<serde_json::Value>,
    /// The device properties.
    pub device_properties: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The ip-address for the vpn-site.
    pub ip_address: Output<serde_json::Value>,
    /// IsSecuritySite flag.
    pub is_security_site: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the VPN site resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The key for vpn-site that can be used for connections.
    pub site_key: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The VirtualWAN to which the vpnSite belongs.
    pub virtual_wan: Output<serde_json::Value>,
    /// List of all vpn site links.
    pub vpn_site_links: Output<serde_json::Value>,
}

impl VpnSite {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20200501:VpnSite";

    /// Create a new VpnSite resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VpnSiteArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.address_space {
            pulumi_sdk::resolve_input("addressSpace", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.bgp_properties {
            pulumi_sdk::resolve_input("bgpProperties", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.device_properties {
            pulumi_sdk::resolve_input("deviceProperties", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_address {
            pulumi_sdk::resolve_input("ipAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_security_site {
            pulumi_sdk::resolve_input("isSecuritySite", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.site_key {
            pulumi_sdk::resolve_input("siteKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_wan {
            pulumi_sdk::resolve_input("virtualWan", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_site_links {
            pulumi_sdk::resolve_input_vec("vpnSiteLinks", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_site_name {
            pulumi_sdk::resolve_input("vpnSiteName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            bgp_properties: registered.outputs.get("bgpProperties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            device_properties: registered.outputs.get("deviceProperties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_address: registered.outputs.get("ipAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_security_site: registered.outputs.get("isSecuritySite")
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
            site_key: registered.outputs.get("siteKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_wan: registered.outputs.get("virtualWan")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_site_links: registered.outputs.get("vpnSiteLinks")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
