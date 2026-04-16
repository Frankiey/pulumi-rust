use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a virtual wan p2s vpn gateway.
#[derive(Default)]
pub struct GetP2sVpnGatewayArgs {
    /// The name of the gateway.
    pub gateway_name: String,
    /// The resource group name of the P2SVpnGateway.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetP2sVpnGatewayResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The reference of the address space resource which represents the custom routes specified by the customer for P2SVpnGateway and P2S VpnClient.
    pub custom_routes: Option<network::v20190701::AddressSpaceResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// The P2SVpnServerConfiguration to which the p2sVpnGateway is attached to.
    pub p2s_vpn_server_configuration: Option<network::v20190701::SubResourceResponse>,
    /// The provisioning state of the P2S VPN gateway resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The VirtualHub to which the gateway belongs.
    pub virtual_hub: Option<network::v20190701::SubResourceResponse>,
    /// The reference of the address space resource which represents Address space for P2S VpnClient.
    pub vpn_client_address_pool: Option<network::v20190701::AddressSpaceResponse>,
    /// All P2S VPN clients' connection health status.
    pub vpn_client_connection_health: network::v20190701::VpnClientConnectionHealthResponse,
    /// The scale unit for this p2s vpn gateway.
    pub vpn_gateway_scale_unit: Option<i64>,
}

/// Retrieves the details of a virtual wan p2s vpn gateway.
pub async fn get_p2s_vpn_gateway(
    ctx: &Context,
    args: GetP2sVpnGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetP2sVpnGatewayResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("gatewayName".to_string(), json!(args.gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20190701:getP2sVpnGateway", invoke_args, &opts).await?;

    Ok(GetP2sVpnGatewayResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        custom_routes: result.fields.get("customRoutes").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        p2s_vpn_server_configuration: result.fields.get("p2SVpnServerConfiguration").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hub: result.fields.get("virtualHub").cloned().map(serde_json::from_value).transpose()?,
        vpn_client_address_pool: result.fields.get("vpnClientAddressPool").cloned().map(serde_json::from_value).transpose()?,
        vpn_client_connection_health: serde_json::from_value(result.fields.get("vpnClientConnectionHealth").cloned().unwrap_or_default())?
            ,
        vpn_gateway_scale_unit: result.fields.get("vpnGatewayScaleUnit").cloned().map(serde_json::from_value).transpose()?,
    })
}
