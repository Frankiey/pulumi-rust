use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets pre-generated VPN profile for P2S client of the virtual network gateway in the specified resource group. The profile needs to be generated first using generateVpnProfile.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayVpnProfilePackageUrlArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayVpnProfilePackageUrlResult {
    pub value: Option<String>,
}

/// Gets pre-generated VPN profile for P2S client of the virtual network gateway in the specified resource group. The profile needs to be generated first using generateVpnProfile.
pub async fn get_virtual_network_gateway_vpn_profile_package_url(
    ctx: &Context,
    args: GetVirtualNetworkGatewayVpnProfilePackageUrlArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayVpnProfilePackageUrlResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20180801:getVirtualNetworkGatewayVpnProfilePackageUrl", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayVpnProfilePackageUrlResult {
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
