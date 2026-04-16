use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a HubVirtualNetworkConnection.
#[derive(Default)]
pub struct GetHubVirtualNetworkConnectionArgs {
    /// The name of the vpn connection.
    pub connection_name: String,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetHubVirtualNetworkConnectionResult {
    /// Deprecated: VirtualHub to RemoteVnet transit to enabled or not.
    pub allow_hub_to_remote_vnet_transit: Option<bool>,
    /// Deprecated: Allow RemoteVnet to use Virtual Hub's gateways.
    pub allow_remote_vnet_to_use_hub_vnet_gateways: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Enable internet security.
    pub enable_internet_security: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the hub virtual network connection resource.
    pub provisioning_state: String,
    /// Reference to the remote virtual network.
    pub remote_virtual_network: Option<network::v20230401::SubResourceResponse>,
    /// The Routing Configuration indicating the associated and propagated route tables on this connection.
    pub routing_configuration: Option<network::v20230401::RoutingConfigurationResponse>,
}

/// Retrieves the details of a HubVirtualNetworkConnection.
pub async fn get_hub_virtual_network_connection(
    ctx: &Context,
    args: GetHubVirtualNetworkConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetHubVirtualNetworkConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230401:getHubVirtualNetworkConnection", invoke_args, &opts).await?;

    Ok(GetHubVirtualNetworkConnectionResult {
        allow_hub_to_remote_vnet_transit: result.fields.get("allowHubToRemoteVnetTransit").cloned().map(serde_json::from_value).transpose()?,
        allow_remote_vnet_to_use_hub_vnet_gateways: result.fields.get("allowRemoteVnetToUseHubVnetGateways").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        enable_internet_security: result.fields.get("enableInternetSecurity").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        remote_virtual_network: result.fields.get("remoteVirtualNetwork").cloned().map(serde_json::from_value).transpose()?,
        routing_configuration: result.fields.get("routingConfiguration").cloned().map(serde_json::from_value).transpose()?,
    })
}
