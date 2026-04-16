use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get specified scope connection created by this Network Manager.
#[derive(Default)]
pub struct GetScopeConnectionArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// Name for the cross-tenant connection.
    pub scope_connection_name: String,
}

/// Result of the function invocation.
pub struct GetScopeConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description of the scope connection.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource name.
    pub name: String,
    /// Resource ID.
    pub resource_id: Option<String>,
    /// The system metadata related to this resource.
    pub system_data: network::v20240501::SystemDataResponse,
    /// Tenant ID.
    pub tenant_id: Option<String>,
    /// Resource type.
    pub type_: String,
}

/// Get specified scope connection created by this Network Manager.
pub async fn get_scope_connection(
    ctx: &Context,
    args: GetScopeConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetScopeConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("scopeConnectionName".to_string(), json!(args.scope_connection_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240501:getScopeConnection", invoke_args, &opts).await?;

    Ok(GetScopeConnectionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        resource_id: result.fields.get("resourceId").cloned().map(serde_json::from_value).transpose()?,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tenant_id: result.fields.get("tenantId").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
