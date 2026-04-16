use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Virtual Router Peering.
#[derive(Default)]
pub struct GetVirtualRouterPeeringArgs {
    /// The name of the Virtual Router Peering.
    pub peering_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the Virtual Router.
    pub virtual_router_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualRouterPeeringResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Name of the virtual router peering that is unique within a virtual router.
    pub name: Option<String>,
    /// Peer ASN.
    pub peer_asn: Option<f64>,
    /// Peer IP.
    pub peer_ip: Option<String>,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Peering type.
    pub type_: String,
}

/// Gets the specified Virtual Router Peering.
pub async fn get_virtual_router_peering(
    ctx: &Context,
    args: GetVirtualRouterPeeringArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualRouterPeeringResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("peeringName".to_string(), json!(args.peering_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualRouterName".to_string(), json!(args.virtual_router_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240501:getVirtualRouterPeering", invoke_args, &opts).await?;

    Ok(GetVirtualRouterPeeringResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
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
