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
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// list of all vnet connections with this VirtualHub.
    pub hub_virtual_network_connections: Option<Vec<network::v20180701::HubVirtualNetworkConnectionResponse>>,
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
    /// The VirtualWAN to which the VirtualHub belongs
    pub virtual_wan: Option<network::v20180701::SubResourceResponse>,
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
    let result = ctx.invoke("azure-native:network/v20180701:getVirtualHub", invoke_args, &opts).await?;

    Ok(GetVirtualHubResult {
        address_prefix: result.fields.get("addressPrefix").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        hub_virtual_network_connections: result.fields.get("hubVirtualNetworkConnections").cloned().map(serde_json::from_value).transpose()?,
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
        virtual_wan: result.fields.get("virtualWan").cloned().map(serde_json::from_value).transpose()?,
    })
}
