use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VirtualHub Resource.
pub struct VirtualHubArgs {
    /// Address-prefix for this VirtualHub.
    pub address_prefix: Option<Input<String>>,
    /// Flag to control transit for VirtualRouter hub.
    pub allow_branch_to_branch_traffic: Option<Input<bool>>,
    /// The azureFirewall associated with this VirtualHub.
    pub azure_firewall: Option<Input<network::v20221101::SubResourceArgs>>,
    /// The expressRouteGateway associated with this VirtualHub.
    pub express_route_gateway: Option<Input<network::v20221101::SubResourceArgs>>,
    /// The hubRoutingPreference of this VirtualHub.
    pub hub_routing_preference: Option<Input<serde_json::Value>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The P2SVpnGateway associated with this VirtualHub.
    pub p2s_vpn_gateway: Option<Input<network::v20221101::SubResourceArgs>>,
    /// The preferred gateway to route on-prem traffic
    pub preferred_routing_gateway: Option<Input<serde_json::Value>>,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: Input<String>,
    /// The routeTable associated with this virtual hub.
    pub route_table: Option<Input<network::v20221101::VirtualHubRouteTableArgs>>,
    /// The securityPartnerProvider associated with this VirtualHub.
    pub security_partner_provider: Option<Input<network::v20221101::SubResourceArgs>>,
    /// The Security Provider name.
    pub security_provider_name: Option<Input<String>>,
    /// The sku of this VirtualHub.
    pub sku: Option<Input<String>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The name of the VirtualHub.
    pub virtual_hub_name: Option<Input<String>>,
    /// List of all virtual hub route table v2s associated with this VirtualHub.
    pub virtual_hub_route_table_v2s: Option<Vec<Input<network::v20221101::VirtualHubRouteTableV2Args>>>,
    /// VirtualRouter ASN.
    pub virtual_router_asn: Option<Input<f64>>,
    /// The VirtualHub Router autoscale configuration.
    pub virtual_router_auto_scale_configuration: Option<Input<network::v20221101::VirtualRouterAutoScaleConfigurationArgs>>,
    /// VirtualRouter IPs.
    pub virtual_router_ips: Option<Vec<Input<String>>>,
    /// The VirtualWAN to which the VirtualHub belongs.
    pub virtual_wan: Option<Input<network::v20221101::SubResourceArgs>>,
    /// The VpnGateway associated with this VirtualHub.
    pub vpn_gateway: Option<Input<network::v20221101::SubResourceArgs>>,
}

/// VirtualHub Resource.
pub struct VirtualHub {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Address-prefix for this VirtualHub.
    pub address_prefix: Output<serde_json::Value>,
    /// Flag to control transit for VirtualRouter hub.
    pub allow_branch_to_branch_traffic: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The azureFirewall associated with this VirtualHub.
    pub azure_firewall: Output<serde_json::Value>,
    /// List of references to Bgp Connections.
    pub bgp_connections: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The expressRouteGateway associated with this VirtualHub.
    pub express_route_gateway: Output<serde_json::Value>,
    /// The hubRoutingPreference of this VirtualHub.
    pub hub_routing_preference: Output<serde_json::Value>,
    /// List of references to IpConfigurations.
    pub ip_configurations: Output<serde_json::Value>,
    /// Kind of service virtual hub. This is metadata used for the Azure portal experience for Route Server.
    pub kind: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The P2SVpnGateway associated with this VirtualHub.
    pub p2s_vpn_gateway: Output<serde_json::Value>,
    /// The preferred gateway to route on-prem traffic
    pub preferred_routing_gateway: Output<serde_json::Value>,
    /// The provisioning state of the virtual hub resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// List of references to RouteMaps.
    pub route_maps: Output<serde_json::Value>,
    /// The routeTable associated with this virtual hub.
    pub route_table: Output<serde_json::Value>,
    /// The routing state.
    pub routing_state: Output<serde_json::Value>,
    /// The securityPartnerProvider associated with this VirtualHub.
    pub security_partner_provider: Output<serde_json::Value>,
    /// The Security Provider name.
    pub security_provider_name: Output<serde_json::Value>,
    /// The sku of this VirtualHub.
    pub sku: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// List of all virtual hub route table v2s associated with this VirtualHub.
    pub virtual_hub_route_table_v2s: Output<serde_json::Value>,
    /// VirtualRouter ASN.
    pub virtual_router_asn: Output<serde_json::Value>,
    /// The VirtualHub Router autoscale configuration.
    pub virtual_router_auto_scale_configuration: Output<serde_json::Value>,
    /// VirtualRouter IPs.
    pub virtual_router_ips: Output<serde_json::Value>,
    /// The VirtualWAN to which the VirtualHub belongs.
    pub virtual_wan: Output<serde_json::Value>,
    /// The VpnGateway associated with this VirtualHub.
    pub vpn_gateway: Output<serde_json::Value>,
}

impl VirtualHub {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20221101:VirtualHub";

    /// Create a new VirtualHub resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualHubArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.address_prefix {
            pulumi_sdk::resolve_input("addressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.allow_branch_to_branch_traffic {
            pulumi_sdk::resolve_input("allowBranchToBranchTraffic", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.azure_firewall {
            pulumi_sdk::resolve_input("azureFirewall", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_gateway {
            pulumi_sdk::resolve_input("expressRouteGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.hub_routing_preference {
            pulumi_sdk::resolve_input("hubRoutingPreference", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.p2s_vpn_gateway {
            pulumi_sdk::resolve_input("p2SVpnGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.preferred_routing_gateway {
            pulumi_sdk::resolve_input("preferredRoutingGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.route_table {
            pulumi_sdk::resolve_input("routeTable", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.security_partner_provider {
            pulumi_sdk::resolve_input("securityPartnerProvider", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.security_provider_name {
            pulumi_sdk::resolve_input("securityProviderName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_hub_name {
            pulumi_sdk::resolve_input("virtualHubName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_hub_route_table_v2s {
            pulumi_sdk::resolve_input_vec("virtualHubRouteTableV2s", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_router_asn {
            pulumi_sdk::resolve_input("virtualRouterAsn", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_router_auto_scale_configuration {
            pulumi_sdk::resolve_input("virtualRouterAutoScaleConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_router_ips {
            pulumi_sdk::resolve_input_vec("virtualRouterIps", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_wan {
            pulumi_sdk::resolve_input("virtualWan", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vpn_gateway {
            pulumi_sdk::resolve_input("vpnGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            address_prefix: registered.outputs.get("addressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_branch_to_branch_traffic: registered.outputs.get("allowBranchToBranchTraffic")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_firewall: registered.outputs.get("azureFirewall")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            bgp_connections: registered.outputs.get("bgpConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_gateway: registered.outputs.get("expressRouteGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            hub_routing_preference: registered.outputs.get("hubRoutingPreference")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            kind: registered.outputs.get("kind")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            p2s_vpn_gateway: registered.outputs.get("p2SVpnGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            preferred_routing_gateway: registered.outputs.get("preferredRoutingGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_maps: registered.outputs.get("routeMaps")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_table: registered.outputs.get("routeTable")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routing_state: registered.outputs.get("routingState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            security_partner_provider: registered.outputs.get("securityPartnerProvider")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            security_provider_name: registered.outputs.get("securityProviderName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_hub_route_table_v2s: registered.outputs.get("virtualHubRouteTableV2s")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_router_asn: registered.outputs.get("virtualRouterAsn")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_router_auto_scale_configuration: registered.outputs.get("virtualRouterAutoScaleConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_router_ips: registered.outputs.get("virtualRouterIps")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_wan: registered.outputs.get("virtualWan")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vpn_gateway: registered.outputs.get("vpnGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
