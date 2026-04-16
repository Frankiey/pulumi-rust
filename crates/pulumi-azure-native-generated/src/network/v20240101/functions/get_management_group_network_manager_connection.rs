use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get a specified connection created by this management group.
#[derive(Default)]
pub struct GetManagementGroupNetworkManagerConnectionArgs {
    /// The management group Id which uniquely identify the Microsoft Azure management group.
    pub management_group_id: String,
    /// Name for the network manager connection.
    pub network_manager_connection_name: String,
}

/// Result of the function invocation.
pub struct GetManagementGroupNetworkManagerConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description of the network manager connection.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// Network Manager Id.
    pub network_manager_id: Option<String>,
    /// The system metadata related to this resource.
    pub system_data: network::v20240101::SystemDataResponse,
    /// Resource type.
    pub type_: String,
}

/// Get a specified connection created by this management group.
pub async fn get_management_group_network_manager_connection(
    ctx: &Context,
    args: GetManagementGroupNetworkManagerConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetManagementGroupNetworkManagerConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("managementGroupId".to_string(), json!(args.management_group_id));
    invoke_args.insert("networkManagerConnectionName".to_string(), json!(args.network_manager_connection_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240101:getManagementGroupNetworkManagerConnection", invoke_args, &opts).await?;

    Ok(GetManagementGroupNetworkManagerConnectionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_manager_id: result.fields.get("networkManagerId").cloned().map(serde_json::from_value).transpose()?,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
