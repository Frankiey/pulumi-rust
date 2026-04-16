use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// This operation retrieves a list of routes the virtual network gateway has learned, including routes learned from BGP peers.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayLearnedRoutesArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayLearnedRoutesResult {
    /// List of gateway routes.
    pub value: Option<Vec<network::v20200801::GatewayRouteResponse>>,
}

/// This operation retrieves a list of routes the virtual network gateway has learned, including routes learned from BGP peers.
pub async fn get_virtual_network_gateway_learned_routes(
    ctx: &Context,
    args: GetVirtualNetworkGatewayLearnedRoutesArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayLearnedRoutesResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200801:getVirtualNetworkGatewayLearnedRoutes", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayLearnedRoutesResult {
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
