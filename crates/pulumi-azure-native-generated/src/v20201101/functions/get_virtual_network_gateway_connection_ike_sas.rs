use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Lists IKE Security Associations for the virtual network gateway connection in the specified resource group.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayConnectionIkeSasArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway Connection.
    pub virtual_network_gateway_connection_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayConnectionIkeSasResult {
    pub value: Option<String>,
}

/// Lists IKE Security Associations for the virtual network gateway connection in the specified resource group.
pub async fn get_virtual_network_gateway_connection_ike_sas(
    ctx: &Context,
    args: GetVirtualNetworkGatewayConnectionIkeSasArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayConnectionIkeSasResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayConnectionName".to_string(), json!(args.virtual_network_gateway_connection_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20201101:getVirtualNetworkGatewayConnectionIkeSas", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayConnectionIkeSasResult {
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
