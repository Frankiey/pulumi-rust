use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// This operation retrieves the route set information for an Express Route Gateway based on their resiliency
#[derive(Default)]
pub struct GetVirtualNetworkGatewayRoutesInformationArgs {
    /// Attempt to recalculate the Route Sets Information for the gateway
    pub attempt_refresh: Option<bool>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayRoutesInformationResult {
    /// Dictionary containing map of the circuit id and circuit details
    pub circuits_metadata_map: Option<HashMap<String, network::v20250301::CircuitMetadataMapResponse>>,
    /// Timestamp denoting the last time when the route sets were computed for the gateway
    pub last_computed_time: Option<String>,
    /// Timestamp denoting the next eligible time to re-compute the route sets for the gateway
    pub next_eligible_compute_time: Option<String>,
    /// Version for the route set
    pub route_set_version: Option<String>,
    /// List of Gateway Route Sets
    pub route_sets: Option<Vec<network::v20250301::GatewayRouteSetResponse>>,
}

/// This operation retrieves the route set information for an Express Route Gateway based on their resiliency
pub async fn get_virtual_network_gateway_routes_information(
    ctx: &Context,
    args: GetVirtualNetworkGatewayRoutesInformationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayRoutesInformationResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.attempt_refresh {
        invoke_args.insert("attemptRefresh".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getVirtualNetworkGatewayRoutesInformation", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayRoutesInformationResult {
        circuits_metadata_map: result.fields.get("circuitsMetadataMap").cloned().map(serde_json::from_value).transpose()?,
        last_computed_time: result.fields.get("lastComputedTime").cloned().map(serde_json::from_value).transpose()?,
        next_eligible_compute_time: result.fields.get("nextEligibleComputeTime").cloned().map(serde_json::from_value).transpose()?,
        route_set_version: result.fields.get("routeSetVersion").cloned().map(serde_json::from_value).transpose()?,
        route_sets: result.fields.get("routeSets").cloned().map(serde_json::from_value).transpose()?,
    })
}
