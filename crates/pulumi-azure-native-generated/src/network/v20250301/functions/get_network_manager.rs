use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Network Manager.
#[derive(Default)]
pub struct GetNetworkManagerArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkManagerResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description of the network manager.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Scope Access.
    pub network_manager_scope_accesses: Option<Vec<String>>,
    /// Scope of Network Manager.
    pub network_manager_scopes: network::v20250301::NetworkManagerPropertiesResponseNetworkManagerScopes,
    /// The provisioning state of the network manager resource.
    pub provisioning_state: String,
    /// Unique identifier for this resource.
    pub resource_guid: String,
    /// The system metadata related to this resource.
    pub system_data: network::v20250301::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified Network Manager.
pub async fn get_network_manager(
    ctx: &Context,
    args: GetNetworkManagerArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkManagerResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250301:getNetworkManager", invoke_args, &opts).await?;

    Ok(GetNetworkManagerResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_manager_scope_accesses: result.fields.get("networkManagerScopeAccesses").cloned().map(serde_json::from_value).transpose()?,
        network_manager_scopes: serde_json::from_value(result.fields.get("networkManagerScopes").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
