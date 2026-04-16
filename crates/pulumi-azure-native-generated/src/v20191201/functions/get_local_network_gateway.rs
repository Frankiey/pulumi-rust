use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified local network gateway in a resource group.
#[derive(Default)]
pub struct GetLocalNetworkGatewayArgs {
    /// The name of the local network gateway.
    pub local_network_gateway_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetLocalNetworkGatewayResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Local network gateway's BGP speaker settings.
    pub bgp_settings: Option<network::v20191201::BgpSettingsResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// FQDN of local network gateway.
    pub fqdn: Option<String>,
    /// IP address of local network gateway.
    pub gateway_ip_address: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// Local network site address space.
    pub local_network_address_space: Option<network::v20191201::AddressSpaceResponse>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the local network gateway resource.
    pub provisioning_state: String,
    /// The resource GUID property of the local network gateway resource.
    pub resource_guid: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified local network gateway in a resource group.
pub async fn get_local_network_gateway(
    ctx: &Context,
    args: GetLocalNetworkGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetLocalNetworkGatewayResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("localNetworkGatewayName".to_string(), json!(args.local_network_gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20191201:getLocalNetworkGateway", invoke_args, &opts).await?;

    Ok(GetLocalNetworkGatewayResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bgp_settings: result.fields.get("bgpSettings").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        fqdn: result.fields.get("fqdn").cloned().map(serde_json::from_value).transpose()?,
        gateway_ip_address: result.fields.get("gatewayIpAddress").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        local_network_address_space: result.fields.get("localNetworkAddressSpace").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
