use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Peering in an ExpressRouteCircuit resource.
pub struct ExpressRouteCircuitPeeringArgs {
    /// The Azure ASN.
    pub azure_asn: Option<Input<i64>>,
    /// The name of the express route circuit.
    pub circuit_name: Input<String>,
    /// The list of circuit connections associated with Azure Private Peering for this circuit.
    pub connections: Option<Vec<Input<network::v20190701::ExpressRouteCircuitConnectionArgs>>>,
    /// The GatewayManager Etag.
    pub gateway_manager_etag: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The IPv6 peering configuration.
    pub ipv6peering_config: Option<Input<network::v20190701::Ipv6ExpressRouteCircuitPeeringConfigArgs>>,
    /// Who was the last to modify the peering.
    pub last_modified_by: Option<Input<String>>,
    /// The Microsoft peering configuration.
    pub microsoft_peering_config: Option<Input<network::v20190701::ExpressRouteCircuitPeeringConfigArgs>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The peer ASN.
    pub peer_asn: Option<Input<f64>>,
    /// The name of the peering.
    pub peering_name: Option<Input<String>>,
    /// The peering type.
    pub peering_type: Option<Input<serde_json::Value>>,
    /// The primary port.
    pub primary_azure_port: Option<Input<String>>,
    /// The primary address prefix.
    pub primary_peer_address_prefix: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The reference of the RouteFilter resource.
    pub route_filter: Option<Input<network::v20190701::SubResourceArgs>>,
    /// The secondary port.
    pub secondary_azure_port: Option<Input<String>>,
    /// The secondary address prefix.
    pub secondary_peer_address_prefix: Option<Input<String>>,
    /// The shared key.
    pub shared_key: Option<Input<String>>,
    /// The peering state.
    pub state: Option<Input<serde_json::Value>>,
    /// The peering stats of express route circuit.
    pub stats: Option<Input<network::v20190701::ExpressRouteCircuitStatsArgs>>,
    /// The VLAN ID.
    pub vlan_id: Option<Input<i64>>,
}

/// Peering in an ExpressRouteCircuit resource.
pub struct ExpressRouteCircuitPeering {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure ASN.
    pub azure_asn: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The list of circuit connections associated with Azure Private Peering for this circuit.
    pub connections: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The ExpressRoute connection.
    pub express_route_connection: Output<serde_json::Value>,
    /// The GatewayManager Etag.
    pub gateway_manager_etag: Output<serde_json::Value>,
    /// The IPv6 peering configuration.
    pub ipv6peering_config: Output<serde_json::Value>,
    /// Who was the last to modify the peering.
    pub last_modified_by: Output<serde_json::Value>,
    /// The Microsoft peering configuration.
    pub microsoft_peering_config: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The peer ASN.
    pub peer_asn: Output<serde_json::Value>,
    /// The list of peered circuit connections associated with Azure Private Peering for this circuit.
    pub peered_connections: Output<serde_json::Value>,
    /// The peering type.
    pub peering_type: Output<serde_json::Value>,
    /// The primary port.
    pub primary_azure_port: Output<serde_json::Value>,
    /// The primary address prefix.
    pub primary_peer_address_prefix: Output<serde_json::Value>,
    /// The provisioning state of the express route circuit peering resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The reference of the RouteFilter resource.
    pub route_filter: Output<serde_json::Value>,
    /// The secondary port.
    pub secondary_azure_port: Output<serde_json::Value>,
    /// The secondary address prefix.
    pub secondary_peer_address_prefix: Output<serde_json::Value>,
    /// The shared key.
    pub shared_key: Output<serde_json::Value>,
    /// The peering state.
    pub state: Output<serde_json::Value>,
    /// The peering stats of express route circuit.
    pub stats: Output<serde_json::Value>,
    /// Type of the resource.
    pub type_: Output<serde_json::Value>,
    /// The VLAN ID.
    pub vlan_id: Output<serde_json::Value>,
}

impl ExpressRouteCircuitPeering {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190701:ExpressRouteCircuitPeering";

    /// Create a new ExpressRouteCircuitPeering resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ExpressRouteCircuitPeeringArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.azure_asn {
            pulumi_sdk::resolve_input("azureASN", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("circuitName", args.circuit_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.connections {
            pulumi_sdk::resolve_input_vec("connections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_manager_etag {
            pulumi_sdk::resolve_input("gatewayManagerEtag", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ipv6peering_config {
            pulumi_sdk::resolve_input("ipv6PeeringConfig", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.last_modified_by {
            pulumi_sdk::resolve_input("lastModifiedBy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.microsoft_peering_config {
            pulumi_sdk::resolve_input("microsoftPeeringConfig", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peer_asn {
            pulumi_sdk::resolve_input("peerASN", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peering_name {
            pulumi_sdk::resolve_input("peeringName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peering_type {
            pulumi_sdk::resolve_input("peeringType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.primary_azure_port {
            pulumi_sdk::resolve_input("primaryAzurePort", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.primary_peer_address_prefix {
            pulumi_sdk::resolve_input("primaryPeerAddressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.route_filter {
            pulumi_sdk::resolve_input("routeFilter", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.secondary_azure_port {
            pulumi_sdk::resolve_input("secondaryAzurePort", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.secondary_peer_address_prefix {
            pulumi_sdk::resolve_input("secondaryPeerAddressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.shared_key {
            pulumi_sdk::resolve_input("sharedKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.state {
            pulumi_sdk::resolve_input("state", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.stats {
            pulumi_sdk::resolve_input("stats", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.vlan_id {
            pulumi_sdk::resolve_input("vlanId", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_asn: registered.outputs.get("azureASN")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            connections: registered.outputs.get("connections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_connection: registered.outputs.get("expressRouteConnection")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            gateway_manager_etag: registered.outputs.get("gatewayManagerEtag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ipv6peering_config: registered.outputs.get("ipv6PeeringConfig")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            last_modified_by: registered.outputs.get("lastModifiedBy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            microsoft_peering_config: registered.outputs.get("microsoftPeeringConfig")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peer_asn: registered.outputs.get("peerASN")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peered_connections: registered.outputs.get("peeredConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peering_type: registered.outputs.get("peeringType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            primary_azure_port: registered.outputs.get("primaryAzurePort")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            primary_peer_address_prefix: registered.outputs.get("primaryPeerAddressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_filter: registered.outputs.get("routeFilter")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            secondary_azure_port: registered.outputs.get("secondaryAzurePort")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            secondary_peer_address_prefix: registered.outputs.get("secondaryPeerAddressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            shared_key: registered.outputs.get("sharedKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            state: registered.outputs.get("state")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            stats: registered.outputs.get("stats")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            vlan_id: registered.outputs.get("vlanId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
