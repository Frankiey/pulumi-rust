use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// The GetBgpPeerStatus operation retrieves the status of all BGP peers.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayBgpPeerStatusArgs {
    /// The IP address of the peer to retrieve the status of.
    pub peer: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayBgpPeerStatusResult {
    /// List of BGP peers
    pub value: Option<Vec<network::v20181001::BgpPeerStatusResponse>>,
}

/// The GetBgpPeerStatus operation retrieves the status of all BGP peers.
pub async fn get_virtual_network_gateway_bgp_peer_status(
    ctx: &Context,
    args: GetVirtualNetworkGatewayBgpPeerStatusArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayBgpPeerStatusResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.peer {
        invoke_args.insert("peer".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20181001:getVirtualNetworkGatewayBgpPeerStatus", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayBgpPeerStatusResult {
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
