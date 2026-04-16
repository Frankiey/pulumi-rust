use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a virtual wan vpn gateway.
#[derive(Default)]
pub struct GetVpnGatewayArgs {
    /// The name of the gateway.
    pub gateway_name: String,
    /// The resource group name of the VpnGateway.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetVpnGatewayResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Local network gateway's BGP speaker settings.
    pub bgp_settings: Option<network::v20180801::BgpSettingsResponse>,
    /// list of all vpn connections to the gateway.
    pub connections: Option<Vec<network::v20180801::VpnConnectionResponse>>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The VirtualHub to which the gateway belongs
    pub virtual_hub: Option<network::v20180801::SubResourceResponse>,
    /// The scale unit for this vpn gateway.
    pub vpn_gateway_scale_unit: Option<i64>,
}

/// Retrieves the details of a virtual wan vpn gateway.
pub async fn get_vpn_gateway(
    ctx: &Context,
    args: GetVpnGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVpnGatewayResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("gatewayName".to_string(), json!(args.gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20180801:getVpnGateway", invoke_args, &opts).await?;

    Ok(GetVpnGatewayResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bgp_settings: result.fields.get("bgpSettings").cloned().map(serde_json::from_value).transpose()?,
        connections: result.fields.get("connections").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hub: result.fields.get("virtualHub").cloned().map(serde_json::from_value).transpose()?,
        vpn_gateway_scale_unit: result.fields.get("vpnGatewayScaleUnit").cloned().map(serde_json::from_value).transpose()?,
    })
}
