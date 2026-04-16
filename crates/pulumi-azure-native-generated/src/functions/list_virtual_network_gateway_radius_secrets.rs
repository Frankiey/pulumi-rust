use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// List all Radius servers with respective radius secrets from virtual network gateway VpnClientConfiguration.
/// 
/// Uses Azure REST API version 2024-10-01.
#[derive(Default)]
pub struct ListVirtualNetworkGatewayRadiusSecretsArgs {
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct ListVirtualNetworkGatewayRadiusSecretsResult {
    /// URL to get the next set of operation list results if there are any.
    pub next_link: Option<String>,
    /// List of Radius servers with respective radius secrets.
    pub value: Option<Vec<network::RadiusAuthServerResponse>>,
}

/// List all Radius servers with respective radius secrets from virtual network gateway VpnClientConfiguration.
pub async fn list_virtual_network_gateway_radius_secrets(
    ctx: &Context,
    args: ListVirtualNetworkGatewayRadiusSecretsArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListVirtualNetworkGatewayRadiusSecretsResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:listVirtualNetworkGatewayRadiusSecrets", invoke_args, &opts).await?;

    Ok(ListVirtualNetworkGatewayRadiusSecretsResult {
        next_link: result.fields.get("nextLink").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
