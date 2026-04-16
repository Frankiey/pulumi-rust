use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get the storage task assignment properties
#[derive(Default)]
pub struct GetStorageTaskAssignmentArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
    /// The name of the storage task assignment within the specified resource group. Storage task assignment names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub storage_task_assignment_name: String,
}

/// Result of the function invocation.
pub struct GetStorageTaskAssignmentResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// Properties of the storage task assignment.
    pub properties: storage::v20250601::StorageTaskAssignmentPropertiesResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Get the storage task assignment properties
pub async fn get_storage_task_assignment(
    ctx: &Context,
    args: GetStorageTaskAssignmentArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetStorageTaskAssignmentResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("storageTaskAssignmentName".to_string(), json!(args.storage_task_assignment_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250601:getStorageTaskAssignment", invoke_args, &opts).await?;

    Ok(GetStorageTaskAssignmentResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
