use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a Virtual Hub Bgp Connection.
#[derive(Default)]
pub struct GetVirtualHubBgpConnectionArgs {
    /// The name of the connection.
    pub connection_name: String,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualHubBgpConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The current state of the VirtualHub to Peer.
    pub connection_state: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The reference to the HubVirtualNetworkConnection resource.
    pub hub_virtual_network_connection: Option<network::v20220901::SubResourceResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// Name of the connection.
    pub name: Option<String>,
    /// Peer ASN.
    pub peer_asn: Option<f64>,
    /// Peer IP.
    pub peer_ip: Option<String>,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Connection type.
    pub type_: String,
}

/// Retrieves the details of a Virtual Hub Bgp Connection.
pub async fn get_virtual_hub_bgp_connection(
    ctx: &Context,
    args: GetVirtualHubBgpConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualHubBgpConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220901:getVirtualHubBgpConnection", invoke_args, &opts).await?;

    Ok(GetVirtualHubBgpConnectionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        connection_state: serde_json::from_value(result.fields.get("connectionState").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        hub_virtual_network_connection: result.fields.get("hubVirtualNetworkConnection").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        peer_asn: result.fields.get("peerAsn").cloned().map(serde_json::from_value).transpose()?,
        peer_ip: result.fields.get("peerIp").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
