use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a VirtualHub.
#[derive(Default)]
pub struct GetVirtualHubArgs {
    /// The resource group name of the VirtualHub.
    pub resource_group_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualHubResult {
    /// Address-prefix for this VirtualHub.
    pub address_prefix: Option<String>,
    /// Flag to control transit for VirtualRouter hub.
    pub allow_branch_to_branch_traffic: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The azureFirewall associated with this VirtualHub.
    pub azure_firewall: Option<network::v20220901::SubResourceResponse>,
    /// List of references to Bgp Connections.
    pub bgp_connections: Vec<network::v20220901::SubResourceResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The expressRouteGateway associated with this VirtualHub.
    pub express_route_gateway: Option<network::v20220901::SubResourceResponse>,
    /// The hubRoutingPreference of this VirtualHub.
    pub hub_routing_preference: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// List of references to IpConfigurations.
    pub ip_configurations: Vec<network::v20220901::SubResourceResponse>,
    /// Kind of service virtual hub. This is metadata used for the Azure portal experience for Route Server.
    pub kind: String,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// The P2SVpnGateway associated with this VirtualHub.
    pub p2s_vpn_gateway: Option<network::v20220901::SubResourceResponse>,
    /// The preferred gateway to route on-prem traffic
    pub preferred_routing_gateway: Option<String>,
    /// The provisioning state of the virtual hub resource.
    pub provisioning_state: String,
    /// List of references to RouteMaps.
    pub route_maps: Vec<network::v20220901::SubResourceResponse>,
    /// The routeTable associated with this virtual hub.
    pub route_table: Option<network::v20220901::VirtualHubRouteTableResponse>,
    /// The routing state.
    pub routing_state: String,
    /// The securityPartnerProvider associated with this VirtualHub.
    pub security_partner_provider: Option<network::v20220901::SubResourceResponse>,
    /// The Security Provider name.
    pub security_provider_name: Option<String>,
    /// The sku of this VirtualHub.
    pub sku: Option<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// List of all virtual hub route table v2s associated with this VirtualHub.
    pub virtual_hub_route_table_v2s: Option<Vec<network::v20220901::VirtualHubRouteTableV2Response>>,
    /// VirtualRouter ASN.
    pub virtual_router_asn: Option<f64>,
    /// The VirtualHub Router autoscale configuration.
    pub virtual_router_auto_scale_configuration: Option<network::v20220901::VirtualRouterAutoScaleConfigurationResponse>,
    /// VirtualRouter IPs.
    pub virtual_router_ips: Option<Vec<String>>,
    /// The VirtualWAN to which the VirtualHub belongs.
    pub virtual_wan: Option<network::v20220901::SubResourceResponse>,
    /// The VpnGateway associated with this VirtualHub.
    pub vpn_gateway: Option<network::v20220901::SubResourceResponse>,
}

/// Retrieves the details of a VirtualHub.
pub async fn get_virtual_hub(
    ctx: &Context,
    args: GetVirtualHubArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualHubResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220901:getVirtualHub", invoke_args, &opts).await?;

    Ok(GetVirtualHubResult {
        address_prefix: result.fields.get("addressPrefix").cloned().map(serde_json::from_value).transpose()?,
        allow_branch_to_branch_traffic: result.fields.get("allowBranchToBranchTraffic").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        azure_firewall: result.fields.get("azureFirewall").cloned().map(serde_json::from_value).transpose()?,
        bgp_connections: serde_json::from_value(result.fields.get("bgpConnections").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        express_route_gateway: result.fields.get("expressRouteGateway").cloned().map(serde_json::from_value).transpose()?,
        hub_routing_preference: result.fields.get("hubRoutingPreference").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: serde_json::from_value(result.fields.get("ipConfigurations").cloned().unwrap_or_default())?
            ,
        kind: serde_json::from_value(result.fields.get("kind").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        p2s_vpn_gateway: result.fields.get("p2SVpnGateway").cloned().map(serde_json::from_value).transpose()?,
        preferred_routing_gateway: result.fields.get("preferredRoutingGateway").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        route_maps: serde_json::from_value(result.fields.get("routeMaps").cloned().unwrap_or_default())?
            ,
        route_table: result.fields.get("routeTable").cloned().map(serde_json::from_value).transpose()?,
        routing_state: serde_json::from_value(result.fields.get("routingState").cloned().unwrap_or_default())?
            ,
        security_partner_provider: result.fields.get("securityPartnerProvider").cloned().map(serde_json::from_value).transpose()?,
        security_provider_name: result.fields.get("securityProviderName").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hub_route_table_v2s: result.fields.get("virtualHubRouteTableV2s").cloned().map(serde_json::from_value).transpose()?,
        virtual_router_asn: result.fields.get("virtualRouterAsn").cloned().map(serde_json::from_value).transpose()?,
        virtual_router_auto_scale_configuration: result.fields.get("virtualRouterAutoScaleConfiguration").cloned().map(serde_json::from_value).transpose()?,
        virtual_router_ips: result.fields.get("virtualRouterIps").cloned().map(serde_json::from_value).transpose()?,
        virtual_wan: result.fields.get("virtualWan").cloned().map(serde_json::from_value).transpose()?,
        vpn_gateway: result.fields.get("vpnGateway").cloned().map(serde_json::from_value).transpose()?,
    })
}
