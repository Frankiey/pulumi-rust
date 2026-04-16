use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets properties of a specified container. 
#[derive(Default)]
pub struct GetBlobContainerArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the blob container within the specified storage account. Blob container names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub container_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetBlobContainerResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Default the container to use specified encryption scope for all writes.
    pub default_encryption_scope: Option<String>,
    /// Indicates whether the blob container was deleted.
    pub deleted: bool,
    /// Blob container deletion time.
    pub deleted_time: String,
    /// Block override of encryption scope from the container default.
    pub deny_encryption_scope_override: Option<bool>,
    /// Enable NFSv3 all squash on blob container.
    pub enable_nfs_v3all_squash: Option<bool>,
    /// Enable NFSv3 root squash on blob container.
    pub enable_nfs_v3root_squash: Option<bool>,
    /// Resource Etag.
    pub etag: String,
    /// The hasImmutabilityPolicy public property is set to true by SRP if ImmutabilityPolicy has been created for this container. The hasImmutabilityPolicy public property is set to false by SRP if ImmutabilityPolicy has not been created for this container.
    pub has_immutability_policy: bool,
    /// The hasLegalHold public property is set to true by SRP if there are at least one existing tag. The hasLegalHold public property is set to false by SRP if all existing legal hold tags are cleared out. There can be a maximum of 1000 blob containers with hasLegalHold=true for a given account.
    pub has_legal_hold: bool,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The ImmutabilityPolicy property of the container.
    pub immutability_policy: storage::v20250101::ImmutabilityPolicyPropertiesResponse,
    /// The object level immutability property of the container. The property is immutable and can only be set to true at the container creation time. Existing containers must undergo a migration process.
    pub immutable_storage_with_versioning: Option<storage::v20250101::ImmutableStorageWithVersioningResponse>,
    /// Returns the date and time the container was last modified.
    pub last_modified_time: String,
    /// Specifies whether the lease on a container is of infinite or fixed duration, only when the container is leased.
    pub lease_duration: String,
    /// Lease state of the container.
    pub lease_state: String,
    /// The lease status of the container.
    pub lease_status: String,
    /// The LegalHold property of the container.
    pub legal_hold: storage::v20250101::LegalHoldPropertiesResponse,
    /// A name-value pair to associate with the container as metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the resource
    pub name: String,
    /// Specifies whether data in the container may be accessed publicly and the level of access.
    pub public_access: Option<String>,
    /// Remaining retention days for soft deleted blob container.
    pub remaining_retention_days: i64,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
    /// The version of the deleted blob container.
    pub version: String,
}

/// Gets properties of a specified container. 
pub async fn get_blob_container(
    ctx: &Context,
    args: GetBlobContainerArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetBlobContainerResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("containerName".to_string(), json!(args.container_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250101:getBlobContainer", invoke_args, &opts).await?;

    Ok(GetBlobContainerResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        default_encryption_scope: result.fields.get("defaultEncryptionScope").cloned().map(serde_json::from_value).transpose()?,
        deleted: serde_json::from_value(result.fields.get("deleted").cloned().unwrap_or_default())?
            ,
        deleted_time: serde_json::from_value(result.fields.get("deletedTime").cloned().unwrap_or_default())?
            ,
        deny_encryption_scope_override: result.fields.get("denyEncryptionScopeOverride").cloned().map(serde_json::from_value).transpose()?,
        enable_nfs_v3all_squash: result.fields.get("enableNfsV3AllSquash").cloned().map(serde_json::from_value).transpose()?,
        enable_nfs_v3root_squash: result.fields.get("enableNfsV3RootSquash").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        has_immutability_policy: serde_json::from_value(result.fields.get("hasImmutabilityPolicy").cloned().unwrap_or_default())?
            ,
        has_legal_hold: serde_json::from_value(result.fields.get("hasLegalHold").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        immutability_policy: serde_json::from_value(result.fields.get("immutabilityPolicy").cloned().unwrap_or_default())?
            ,
        immutable_storage_with_versioning: result.fields.get("immutableStorageWithVersioning").cloned().map(serde_json::from_value).transpose()?,
        last_modified_time: serde_json::from_value(result.fields.get("lastModifiedTime").cloned().unwrap_or_default())?
            ,
        lease_duration: serde_json::from_value(result.fields.get("leaseDuration").cloned().unwrap_or_default())?
            ,
        lease_state: serde_json::from_value(result.fields.get("leaseState").cloned().unwrap_or_default())?
            ,
        lease_status: serde_json::from_value(result.fields.get("leaseStatus").cloned().unwrap_or_default())?
            ,
        legal_hold: serde_json::from_value(result.fields.get("legalHold").cloned().unwrap_or_default())?
            ,
        metadata: result.fields.get("metadata").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        public_access: result.fields.get("publicAccess").cloned().map(serde_json::from_value).transpose()?,
        remaining_retention_days: serde_json::from_value(result.fields.get("remainingRetentionDays").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        version: serde_json::from_value(result.fields.get("version").cloned().unwrap_or_default())?
            ,
    })
}
