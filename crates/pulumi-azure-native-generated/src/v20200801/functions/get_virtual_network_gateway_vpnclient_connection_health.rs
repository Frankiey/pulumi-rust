use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get VPN client connection health detail per P2S client connection of the virtual network gateway in the specified resource group.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayVpnclientConnectionHealthArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayVpnclientConnectionHealthResult {
    /// List of vpn client connection health.
    pub value: Option<Vec<network::v20200801::VpnClientConnectionHealthDetailResponse>>,
}

/// Get VPN client connection health detail per P2S client connection of the virtual network gateway in the specified resource group.
pub async fn get_virtual_network_gateway_vpnclient_connection_health(
    ctx: &Context,
    args: GetVirtualNetworkGatewayVpnclientConnectionHealthArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayVpnclientConnectionHealthResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200801:getVirtualNetworkGatewayVpnclientConnectionHealth", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayVpnclientConnectionHealthResult {
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
