use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified network watcher by resource group.
#[derive(Default)]
pub struct GetNetworkWatcherArgs {
    /// The name of the network watcher.
    pub network_watcher_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkWatcherResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified network watcher by resource group.
pub async fn get_network_watcher(
    ctx: &Context,
    args: GetNetworkWatcherArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkWatcherResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkWatcherName".to_string(), json!(args.network_watcher_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20181201:getNetworkWatcher", invoke_args, &opts).await?;

    Ok(GetNetworkWatcherResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
