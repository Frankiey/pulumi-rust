use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Instance of Verifier Workspace.
/// 
/// Uses Azure REST API version 2024-05-01.
#[derive(Default)]
pub struct GetVerifierWorkspaceArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// Workspace name.
    pub workspace_name: String,
}

/// Result of the function invocation.
pub struct GetVerifierWorkspaceResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The geo-location where the resource lives
    pub location: String,
    /// The name of the resource
    pub name: String,
    /// Properties of Verifier Workspace resource.
    pub properties: network::VerifierWorkspacePropertiesResponse,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Instance of Verifier Workspace.
pub async fn get_verifier_workspace(
    ctx: &Context,
    args: GetVerifierWorkspaceArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVerifierWorkspaceResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("workspaceName".to_string(), json!(args.workspace_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:getVerifierWorkspace", invoke_args, &opts).await?;

    Ok(GetVerifierWorkspaceResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
