use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the blob inventory policy associated with the specified storage account.
#[derive(Default)]
pub struct GetBlobInventoryPolicyArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the storage account blob inventory policy. It should always be 'default'
    pub blob_inventory_policy_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetBlobInventoryPolicyResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// Returns the last modified date and time of the blob inventory policy.
    pub last_modified_time: String,
    /// The name of the resource
    pub name: String,
    /// The storage account blob inventory policy object. It is composed of policy rules.
    pub policy: storage::v20240101::BlobInventoryPolicySchemaResponse,
    /// Metadata pertaining to creation and last modification of the resource.
    pub system_data: storage::v20240101::SystemDataResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the blob inventory policy associated with the specified storage account.
pub async fn get_blob_inventory_policy(
    ctx: &Context,
    args: GetBlobInventoryPolicyArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetBlobInventoryPolicyResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("blobInventoryPolicyName".to_string(), json!(args.blob_inventory_policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20240101:getBlobInventoryPolicy", invoke_args, &opts).await?;

    Ok(GetBlobInventoryPolicyResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        last_modified_time: serde_json::from_value(result.fields.get("lastModifiedTime").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        policy: serde_json::from_value(result.fields.get("policy").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
