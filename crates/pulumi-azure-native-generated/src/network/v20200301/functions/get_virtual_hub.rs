use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a VirtualHub.
#[derive(Default)]
pub struct GetVirtualHubArgs {
    /// The resource group name of the VirtualHub.
    pub resource_group_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualHubResult {
    /// Address-prefix for this VirtualHub.
    pub address_prefix: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The azureFirewall associated with this VirtualHub.
    pub azure_firewall: Option<network::v20200301::SubResourceResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The expressRouteGateway associated with this VirtualHub.
    pub express_route_gateway: Option<network::v20200301::SubResourceResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: String,
    /// Resource name.
    pub name: String,
    /// The P2SVpnGateway associated with this VirtualHub.
    pub p2s_vpn_gateway: Option<network::v20200301::SubResourceResponse>,
    /// The provisioning state of the virtual hub resource.
    pub provisioning_state: String,
    /// The routeTable associated with this virtual hub.
    pub route_table: Option<network::v20200301::VirtualHubRouteTableResponse>,
    /// The securityPartnerProvider associated with this VirtualHub.
    pub security_partner_provider: Option<network::v20200301::SubResourceResponse>,
    /// The Security Provider name.
    pub security_provider_name: Option<String>,
    /// The sku of this VirtualHub.
    pub sku: Option<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// List of all virtual hub route table v2s associated with this VirtualHub.
    pub virtual_hub_route_table_v2s: Option<Vec<network::v20200301::VirtualHubRouteTableV2Response>>,
    /// List of all vnet connections with this VirtualHub.
    pub virtual_network_connections: Option<Vec<network::v20200301::HubVirtualNetworkConnectionResponse>>,
    /// The VirtualWAN to which the VirtualHub belongs.
    pub virtual_wan: Option<network::v20200301::SubResourceResponse>,
    /// The VpnGateway associated with this VirtualHub.
    pub vpn_gateway: Option<network::v20200301::SubResourceResponse>,
}

/// Retrieves the details of a VirtualHub.
pub async fn get_virtual_hub(
    ctx: &Context,
    args: GetVirtualHubArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualHubResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200301:getVirtualHub", invoke_args, &opts).await?;

    Ok(GetVirtualHubResult {
        address_prefix: result.fields.get("addressPrefix").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        azure_firewall: result.fields.get("azureFirewall").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        express_route_gateway: result.fields.get("expressRouteGateway").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        p2s_vpn_gateway: result.fields.get("p2SVpnGateway").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        route_table: result.fields.get("routeTable").cloned().map(serde_json::from_value).transpose()?,
        security_partner_provider: result.fields.get("securityPartnerProvider").cloned().map(serde_json::from_value).transpose()?,
        security_provider_name: result.fields.get("securityProviderName").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hub_route_table_v2s: result.fields.get("virtualHubRouteTableV2s").cloned().map(serde_json::from_value).transpose()?,
        virtual_network_connections: result.fields.get("virtualNetworkConnections").cloned().map(serde_json::from_value).transpose()?,
        virtual_wan: result.fields.get("virtualWan").cloned().map(serde_json::from_value).transpose()?,
        vpn_gateway: result.fields.get("vpnGateway").cloned().map(serde_json::from_value).transpose()?,
    })
}
