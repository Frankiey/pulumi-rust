use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the existing immutability policy along with the corresponding ETag in response headers and body.
#[derive(Default)]
pub struct GetBlobContainerImmutabilityPolicyArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the blob container within the specified storage account. Blob container names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub container_name: String,
    /// The name of the blob container immutabilityPolicy within the specified storage account. ImmutabilityPolicy Name must be 'default'
    pub immutability_policy_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetBlobContainerImmutabilityPolicyResult {
    /// This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to an append blob while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API.
    pub allow_protected_append_writes: Option<bool>,
    /// This property can only be changed for unlocked time-based retention policies. When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining immutability protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted. This property cannot be changed with ExtendImmutabilityPolicy API. The 'allowProtectedAppendWrites' and 'allowProtectedAppendWritesAll' properties are mutually exclusive.
    pub allow_protected_append_writes_all: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Resource Etag.
    pub etag: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The immutability period for the blobs in the container since the policy creation, in days.
    pub immutability_period_since_creation_in_days: Option<i64>,
    /// The name of the resource
    pub name: String,
    /// The ImmutabilityPolicy state of a blob container, possible values include: Locked and Unlocked.
    pub state: String,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the existing immutability policy along with the corresponding ETag in response headers and body.
pub async fn get_blob_container_immutability_policy(
    ctx: &Context,
    args: GetBlobContainerImmutabilityPolicyArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetBlobContainerImmutabilityPolicyResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("containerName".to_string(), json!(args.container_name));
    invoke_args.insert("immutabilityPolicyName".to_string(), json!(args.immutability_policy_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250101:getBlobContainerImmutabilityPolicy", invoke_args, &opts).await?;

    Ok(GetBlobContainerImmutabilityPolicyResult {
        allow_protected_append_writes: result.fields.get("allowProtectedAppendWrites").cloned().map(serde_json::from_value).transpose()?,
        allow_protected_append_writes_all: result.fields.get("allowProtectedAppendWritesAll").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        immutability_period_since_creation_in_days: result.fields.get("immutabilityPeriodSinceCreationInDays").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        state: serde_json::from_value(result.fields.get("state").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
