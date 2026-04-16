use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Express Route Circuit Connection from the specified express route circuit.
/// 
/// Uses Azure REST API version 2024-05-01.
#[derive(Default)]
pub struct GetExpressRouteCircuitConnectionArgs {
    /// The name of the express route circuit.
    pub circuit_name: String,
    /// The name of the express route circuit connection.
    pub connection_name: String,
    /// The name of the peering.
    pub peering_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRouteCircuitConnectionResult {
    /// /29 IP address space to carve out Customer addresses for tunnels.
    pub address_prefix: Option<String>,
    /// The authorization key.
    pub authorization_key: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Express Route Circuit connection state.
    pub circuit_connection_status: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Reference to Express Route Circuit Private Peering Resource of the circuit initiating connection.
    pub express_route_circuit_peering: Option<network::SubResourceResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// IPv6 Address PrefixProperties of the express route circuit connection.
    pub ipv6circuit_connection_config: Option<network::Ipv6CircuitConnectionConfigResponse>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Reference to Express Route Circuit Private Peering Resource of the peered circuit.
    pub peer_express_route_circuit_peering: Option<network::SubResourceResponse>,
    /// The provisioning state of the express route circuit connection resource.
    pub provisioning_state: String,
    /// Type of the resource.
    pub type_: String,
}

/// Gets the specified Express Route Circuit Connection from the specified express route circuit.
pub async fn get_express_route_circuit_connection(
    ctx: &Context,
    args: GetExpressRouteCircuitConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRouteCircuitConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("circuitName".to_string(), json!(args.circuit_name));
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("peeringName".to_string(), json!(args.peering_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:getExpressRouteCircuitConnection", invoke_args, &opts).await?;

    Ok(GetExpressRouteCircuitConnectionResult {
        address_prefix: result.fields.get("addressPrefix").cloned().map(serde_json::from_value).transpose()?,
        authorization_key: result.fields.get("authorizationKey").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        circuit_connection_status: serde_json::from_value(result.fields.get("circuitConnectionStatus").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        express_route_circuit_peering: result.fields.get("expressRouteCircuitPeering").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ipv6circuit_connection_config: result.fields.get("ipv6CircuitConnectionConfig").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        peer_express_route_circuit_peering: result.fields.get("peerExpressRouteCircuitPeering").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
