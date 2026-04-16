use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the properties of a storage account’s Blob service, including properties for Storage Analytics and CORS (Cross-Origin Resource Sharing) rules.
#[derive(Default)]
pub struct GetBlobServicePropertiesArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the blob Service within the specified storage account. Blob Service Name must be 'default'
    pub blob_services_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetBlobServicePropertiesResult {
    /// Deprecated in favor of isVersioningEnabled property.
    pub automatic_snapshot_policy_enabled: Option<bool>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The blob service properties for change feed events.
    pub change_feed: Option<storage::v20250101::ChangeFeedResponse>,
    /// The blob service properties for container soft delete.
    pub container_delete_retention_policy: Option<storage::v20250101::DeleteRetentionPolicyResponse>,
    /// Specifies CORS rules for the Blob service. You can include up to five CorsRule elements in the request. If no CorsRule elements are included in the request body, all CORS rules will be deleted, and CORS will be disabled for the Blob service.
    pub cors: Option<storage::v20250101::CorsRulesResponse>,
    /// DefaultServiceVersion indicates the default version to use for requests to the Blob service if an incoming request’s version is not specified. Possible values include version 2008-10-27 and all more recent versions.
    pub default_service_version: Option<String>,
    /// The blob service properties for blob soft delete.
    pub delete_retention_policy: Option<storage::v20250101::DeleteRetentionPolicyResponse>,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// Versioning is enabled if set to true.
    pub is_versioning_enabled: Option<bool>,
    /// The blob service property to configure last access time based tracking policy.
    pub last_access_time_tracking_policy: Option<storage::v20250101::LastAccessTimeTrackingPolicyResponse>,
    /// The name of the resource
    pub name: String,
    /// The blob service properties for blob restore policy.
    pub restore_policy: Option<storage::v20250101::RestorePolicyPropertiesResponse>,
    /// Sku name and tier.
    pub sku: storage::v20250101::SkuResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the properties of a storage account’s Blob service, including properties for Storage Analytics and CORS (Cross-Origin Resource Sharing) rules.
pub async fn get_blob_service_properties(
    ctx: &Context,
    args: GetBlobServicePropertiesArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetBlobServicePropertiesResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("blobServicesName".to_string(), json!(args.blob_services_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20250101:getBlobServiceProperties", invoke_args, &opts).await?;

    Ok(GetBlobServicePropertiesResult {
        automatic_snapshot_policy_enabled: result.fields.get("automaticSnapshotPolicyEnabled").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        change_feed: result.fields.get("changeFeed").cloned().map(serde_json::from_value).transpose()?,
        container_delete_retention_policy: result.fields.get("containerDeleteRetentionPolicy").cloned().map(serde_json::from_value).transpose()?,
        cors: result.fields.get("cors").cloned().map(serde_json::from_value).transpose()?,
        default_service_version: result.fields.get("defaultServiceVersion").cloned().map(serde_json::from_value).transpose()?,
        delete_retention_policy: result.fields.get("deleteRetentionPolicy").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        is_versioning_enabled: result.fields.get("isVersioningEnabled").cloned().map(serde_json::from_value).transpose()?,
        last_access_time_tracking_policy: result.fields.get("lastAccessTimeTrackingPolicy").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        restore_policy: result.fields.get("restorePolicy").cloned().map(serde_json::from_value).transpose()?,
        sku: serde_json::from_value(result.fields.get("sku").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
