use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified peering for the express route circuit.
#[derive(Default)]
pub struct GetExpressRouteCircuitPeeringArgs {
    /// The name of the express route circuit.
    pub circuit_name: String,
    /// The name of the peering.
    pub peering_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRouteCircuitPeeringResult {
    /// The Azure ASN.
    pub azure_asn: Option<i64>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The list of circuit connections associated with Azure Private Peering for this circuit.
    pub connections: Option<Vec<network::v20181201::ExpressRouteCircuitConnectionResponse>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The ExpressRoute connection.
    pub express_route_connection: Option<network::v20181201::ExpressRouteConnectionIdResponse>,
    /// The GatewayManager Etag.
    pub gateway_manager_etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// The IPv6 peering configuration.
    pub ipv6peering_config: Option<network::v20181201::Ipv6ExpressRouteCircuitPeeringConfigResponse>,
    /// Gets whether the provider or the customer last modified the peering.
    pub last_modified_by: Option<String>,
    /// The Microsoft peering configuration.
    pub microsoft_peering_config: Option<network::v20181201::ExpressRouteCircuitPeeringConfigResponse>,
    /// Gets name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The peer ASN.
    pub peer_asn: Option<f64>,
    /// The list of peered circuit connections associated with Azure Private Peering for this circuit.
    pub peered_connections: Vec<network::v20181201::PeerExpressRouteCircuitConnectionResponse>,
    /// The peering type.
    pub peering_type: Option<String>,
    /// The primary port.
    pub primary_azure_port: Option<String>,
    /// The primary address prefix.
    pub primary_peer_address_prefix: Option<String>,
    /// Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
    /// The reference of the RouteFilter resource.
    pub route_filter: Option<network::v20181201::RouteFilterResponse>,
    /// The secondary port.
    pub secondary_azure_port: Option<String>,
    /// The secondary address prefix.
    pub secondary_peer_address_prefix: Option<String>,
    /// The shared key.
    pub shared_key: Option<String>,
    /// The peering state.
    pub state: Option<String>,
    /// Gets peering stats.
    pub stats: Option<network::v20181201::ExpressRouteCircuitStatsResponse>,
    /// The VLAN ID.
    pub vlan_id: Option<i64>,
}

/// Gets the specified peering for the express route circuit.
pub async fn get_express_route_circuit_peering(
    ctx: &Context,
    args: GetExpressRouteCircuitPeeringArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRouteCircuitPeeringResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("circuitName".to_string(), json!(args.circuit_name));
    invoke_args.insert("peeringName".to_string(), json!(args.peering_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20181201:getExpressRouteCircuitPeering", invoke_args, &opts).await?;

    Ok(GetExpressRouteCircuitPeeringResult {
        azure_asn: result.fields.get("azureASN").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        connections: result.fields.get("connections").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        express_route_connection: result.fields.get("expressRouteConnection").cloned().map(serde_json::from_value).transpose()?,
        gateway_manager_etag: result.fields.get("gatewayManagerEtag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ipv6peering_config: result.fields.get("ipv6PeeringConfig").cloned().map(serde_json::from_value).transpose()?,
        last_modified_by: result.fields.get("lastModifiedBy").cloned().map(serde_json::from_value).transpose()?,
        microsoft_peering_config: result.fields.get("microsoftPeeringConfig").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        peer_asn: result.fields.get("peerASN").cloned().map(serde_json::from_value).transpose()?,
        peered_connections: serde_json::from_value(result.fields.get("peeredConnections").cloned().unwrap_or_default())?
            ,
        peering_type: result.fields.get("peeringType").cloned().map(serde_json::from_value).transpose()?,
        primary_azure_port: result.fields.get("primaryAzurePort").cloned().map(serde_json::from_value).transpose()?,
        primary_peer_address_prefix: result.fields.get("primaryPeerAddressPrefix").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
        route_filter: result.fields.get("routeFilter").cloned().map(serde_json::from_value).transpose()?,
        secondary_azure_port: result.fields.get("secondaryAzurePort").cloned().map(serde_json::from_value).transpose()?,
        secondary_peer_address_prefix: result.fields.get("secondaryPeerAddressPrefix").cloned().map(serde_json::from_value).transpose()?,
        shared_key: result.fields.get("sharedKey").cloned().map(serde_json::from_value).transpose()?,
        state: result.fields.get("state").cloned().map(serde_json::from_value).transpose()?,
        stats: result.fields.get("stats").cloned().map(serde_json::from_value).transpose()?,
        vlan_id: result.fields.get("vlanId").cloned().map(serde_json::from_value).transpose()?,
    })
}
