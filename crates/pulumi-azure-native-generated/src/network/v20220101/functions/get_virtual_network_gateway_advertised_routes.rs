use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// This operation retrieves a list of routes the virtual network gateway is advertising to the specified peer.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayAdvertisedRoutesArgs {
    /// The IP address of the peer.
    pub peer: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayAdvertisedRoutesResult {
    /// List of gateway routes.
    pub value: Option<Vec<network::v20220101::GatewayRouteResponse>>,
}

/// This operation retrieves a list of routes the virtual network gateway is advertising to the specified peer.
pub async fn get_virtual_network_gateway_advertised_routes(
    ctx: &Context,
    args: GetVirtualNetworkGatewayAdvertisedRoutesArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayAdvertisedRoutesResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("peer".to_string(), json!(args.peer));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220101:getVirtualNetworkGatewayAdvertisedRoutes", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayAdvertisedRoutesResult {
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
