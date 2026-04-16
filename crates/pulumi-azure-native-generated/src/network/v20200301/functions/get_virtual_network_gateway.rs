use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified virtual network gateway by resource group.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayResult {
    /// ActiveActive flag.
    pub active_active: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Virtual network gateway's BGP speaker settings.
    pub bgp_settings: Option<network::v20200301::BgpSettingsResponse>,
    /// The reference to the address space resource which represents the custom routes address space specified by the customer for virtual network gateway and VpnClient.
    pub custom_routes: Option<network::v20200301::AddressSpaceResponse>,
    /// Whether BGP is enabled for this virtual network gateway or not.
    pub enable_bgp: Option<bool>,
    /// Whether dns forwarding is enabled or not.
    pub enable_dns_forwarding: Option<bool>,
    /// Whether private IP needs to be enabled on this gateway for connections or not.
    pub enable_private_ip_address: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The reference to the LocalNetworkGateway resource which represents local network site having default routes. Assign Null value in case of removing existing default site setting.
    pub gateway_default_site: Option<network::v20200301::SubResourceResponse>,
    /// The type of this virtual network gateway.
    pub gateway_type: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// The IP address allocated by the gateway to which dns requests can be sent.
    pub inbound_dns_forwarding_endpoint: String,
    /// IP configurations for virtual network gateway.
    pub ip_configurations: Option<Vec<network::v20200301::VirtualNetworkGatewayIPConfigurationResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the virtual network gateway resource.
    pub provisioning_state: String,
    /// The resource GUID property of the virtual network gateway resource.
    pub resource_guid: String,
    /// The reference to the VirtualNetworkGatewaySku resource which represents the SKU selected for Virtual network gateway.
    pub sku: Option<network::v20200301::VirtualNetworkGatewaySkuResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The reference to the VpnClientConfiguration resource which represents the P2S VpnClient configurations.
    pub vpn_client_configuration: Option<network::v20200301::VpnClientConfigurationResponse>,
    /// The generation for this VirtualNetworkGateway. Must be None if gatewayType is not VPN.
    pub vpn_gateway_generation: Option<String>,
    /// The type of this virtual network gateway.
    pub vpn_type: Option<String>,
}

/// Gets the specified virtual network gateway by resource group.
pub async fn get_virtual_network_gateway(
    ctx: &Context,
    args: GetVirtualNetworkGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200301:getVirtualNetworkGateway", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayResult {
        active_active: result.fields.get("activeActive").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bgp_settings: result.fields.get("bgpSettings").cloned().map(serde_json::from_value).transpose()?,
        custom_routes: result.fields.get("customRoutes").cloned().map(serde_json::from_value).transpose()?,
        enable_bgp: result.fields.get("enableBgp").cloned().map(serde_json::from_value).transpose()?,
        enable_dns_forwarding: result.fields.get("enableDnsForwarding").cloned().map(serde_json::from_value).transpose()?,
        enable_private_ip_address: result.fields.get("enablePrivateIpAddress").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        gateway_default_site: result.fields.get("gatewayDefaultSite").cloned().map(serde_json::from_value).transpose()?,
        gateway_type: result.fields.get("gatewayType").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        inbound_dns_forwarding_endpoint: serde_json::from_value(result.fields.get("inboundDnsForwardingEndpoint").cloned().unwrap_or_default())?
            ,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        vpn_client_configuration: result.fields.get("vpnClientConfiguration").cloned().map(serde_json::from_value).transpose()?,
        vpn_gateway_generation: result.fields.get("vpnGatewayGeneration").cloned().map(serde_json::from_value).transpose()?,
        vpn_type: result.fields.get("vpnType").cloned().map(serde_json::from_value).transpose()?,
    })
}
