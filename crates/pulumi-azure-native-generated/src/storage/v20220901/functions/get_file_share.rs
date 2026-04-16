use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets properties of a specified share.
#[derive(Default)]
pub struct GetFileShareArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// Optional, used to expand the properties within share's properties. Valid values are: stats. Should be passed as a string with delimiter ','.
    pub expand: Option<String>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
    /// The name of the file share within the specified storage account. File share names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub share_name: String,
}

/// Result of the function invocation.
pub struct GetFileShareResult {
    /// Access tier for specific share. GpV2 account can choose between TransactionOptimized (default), Hot, and Cool. FileStorage account can choose Premium.
    pub access_tier: Option<String>,
    /// Indicates the last modification time for share access tier.
    pub access_tier_change_time: String,
    /// Indicates if there is a pending transition for access tier.
    pub access_tier_status: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Indicates whether the share was deleted.
    pub deleted: bool,
    /// The deleted time if the share was deleted.
    pub deleted_time: String,
    /// The authentication protocol that is used for the file share. Can only be specified when creating a share.
    pub enabled_protocols: Option<String>,
    /// Resource Etag.
    pub etag: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// Returns the date and time the share was last modified.
    pub last_modified_time: String,
    /// Specifies whether the lease on a share is of infinite or fixed duration, only when the share is leased.
    pub lease_duration: String,
    /// Lease state of the share.
    pub lease_state: String,
    /// The lease status of the share.
    pub lease_status: String,
    /// A name-value pair to associate with the share as metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the resource
    pub name: String,
    /// Remaining retention days for share that was soft deleted.
    pub remaining_retention_days: i64,
    /// The property is for NFS share only. The default is NoRootSquash.
    pub root_squash: Option<String>,
    /// The maximum size of the share, in gigabytes. Must be greater than 0, and less than or equal to 5TB (5120). For Large File Shares, the maximum size is 102400.
    pub share_quota: Option<i64>,
    /// The approximate size of the data stored on the share. Note that this value may not include all recently created or recently resized files.
    pub share_usage_bytes: f64,
    /// List of stored access policies specified on the share.
    pub signed_identifiers: Option<Vec<storage::v20220901::SignedIdentifierResponse>>,
    /// Creation time of share snapshot returned in the response of list shares with expand param "snapshots".
    pub snapshot_time: String,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
    /// The version of the share.
    pub version: String,
}

/// Gets properties of a specified share.
pub async fn get_file_share(
    ctx: &Context,
    args: GetFileShareArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetFileShareResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("shareName".to_string(), json!(args.share_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20220901:getFileShare", invoke_args, &opts).await?;

    Ok(GetFileShareResult {
        access_tier: result.fields.get("accessTier").cloned().map(serde_json::from_value).transpose()?,
        access_tier_change_time: serde_json::from_value(result.fields.get("accessTierChangeTime").cloned().unwrap_or_default())?
            ,
        access_tier_status: serde_json::from_value(result.fields.get("accessTierStatus").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        deleted: serde_json::from_value(result.fields.get("deleted").cloned().unwrap_or_default())?
            ,
        deleted_time: serde_json::from_value(result.fields.get("deletedTime").cloned().unwrap_or_default())?
            ,
        enabled_protocols: result.fields.get("enabledProtocols").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        last_modified_time: serde_json::from_value(result.fields.get("lastModifiedTime").cloned().unwrap_or_default())?
            ,
        lease_duration: serde_json::from_value(result.fields.get("leaseDuration").cloned().unwrap_or_default())?
            ,
        lease_state: serde_json::from_value(result.fields.get("leaseState").cloned().unwrap_or_default())?
            ,
        lease_status: serde_json::from_value(result.fields.get("leaseStatus").cloned().unwrap_or_default())?
            ,
        metadata: result.fields.get("metadata").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        remaining_retention_days: serde_json::from_value(result.fields.get("remainingRetentionDays").cloned().unwrap_or_default())?
            ,
        root_squash: result.fields.get("rootSquash").cloned().map(serde_json::from_value).transpose()?,
        share_quota: result.fields.get("shareQuota").cloned().map(serde_json::from_value).transpose()?,
        share_usage_bytes: serde_json::from_value(result.fields.get("shareUsageBytes").cloned().unwrap_or_default())?
            ,
        signed_identifiers: result.fields.get("signedIdentifiers").cloned().map(serde_json::from_value).transpose()?,
        snapshot_time: serde_json::from_value(result.fields.get("snapshotTime").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        version: serde_json::from_value(result.fields.get("version").cloned().unwrap_or_default())?
            ,
    })
}
