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
    /// The name of the resource group. The name is case insensitive.
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
    /// File Share Paid Bursting properties.
    pub file_share_paid_bursting: Option<storage::v20250601::FileSharePropertiesFileSharePaidBurstingResponse>,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The calculated burst IOPS of the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub included_burst_iops: i64,
    /// Returns the date and time the share was last modified.
    pub last_modified_time: String,
    /// Specifies whether the lease on a share is of infinite or fixed duration, only when the share is leased.
    pub lease_duration: String,
    /// Lease state of the share.
    pub lease_state: String,
    /// The lease status of the share.
    pub lease_status: String,
    /// The calculated maximum burst credits for the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub max_burst_credits_for_iops: f64,
    /// A name-value pair to associate with the share as metadata.
    pub metadata: Option<HashMap<String, String>>,
    /// The name of the resource
    pub name: String,
    /// Returns the next allowed provisioned bandwidth downgrade time for the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub next_allowed_provisioned_bandwidth_downgrade_time: String,
    /// Returns the next allowed provisioned IOPS downgrade time for the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub next_allowed_provisioned_iops_downgrade_time: String,
    /// Returns the next allowed provisioned storage size downgrade time for the share. This property is only for file shares created under Files Provisioned v1 SSD and Files Provisioned v2 account type
    pub next_allowed_quota_downgrade_time: String,
    /// The provisioned bandwidth of the share, in mebibytes per second. This property is only for file shares created under Files Provisioned v2 account type. Please refer to the GetFileServiceUsage API response for the minimum and maximum allowed value for provisioned bandwidth.
    pub provisioned_bandwidth_mibps: Option<i64>,
    /// The provisioned IOPS of the share. This property is only for file shares created under Files Provisioned v2 account type. Please refer to the GetFileServiceUsage API response for the minimum and maximum allowed value for provisioned IOPS.
    pub provisioned_iops: Option<i64>,
    /// Remaining retention days for share that was soft deleted.
    pub remaining_retention_days: i64,
    /// The property is for NFS share only. The default is NoRootSquash.
    pub root_squash: Option<String>,
    /// The provisioned size of the share, in gibibytes. Must be greater than 0, and less than or equal to 5TB (5120). For Large File Shares, the maximum size is 102400. For file shares created under Files Provisioned v2 account type, please refer to the GetFileServiceUsage API response for the minimum and maximum allowed provisioned storage size.
    pub share_quota: Option<i64>,
    /// The approximate size of the data stored on the share. Note that this value may not include all recently created or recently resized files.
    pub share_usage_bytes: f64,
    /// List of stored access policies specified on the share.
    pub signed_identifiers: Option<Vec<storage::v20250601::SignedIdentifierResponse>>,
    /// Creation time of share snapshot returned in the response of list shares with expand param "snapshots".
    pub snapshot_time: String,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: storage::v20250601::SystemDataResponse,
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
    let result = ctx.invoke("azure-native:storage/v20250601:getFileShare", invoke_args, &opts).await?;

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
        file_share_paid_bursting: result.fields.get("fileSharePaidBursting").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        included_burst_iops: serde_json::from_value(result.fields.get("includedBurstIops").cloned().unwrap_or_default())?
            ,
        last_modified_time: serde_json::from_value(result.fields.get("lastModifiedTime").cloned().unwrap_or_default())?
            ,
        lease_duration: serde_json::from_value(result.fields.get("leaseDuration").cloned().unwrap_or_default())?
            ,
        lease_state: serde_json::from_value(result.fields.get("leaseState").cloned().unwrap_or_default())?
            ,
        lease_status: serde_json::from_value(result.fields.get("leaseStatus").cloned().unwrap_or_default())?
            ,
        max_burst_credits_for_iops: serde_json::from_value(result.fields.get("maxBurstCreditsForIops").cloned().unwrap_or_default())?
            ,
        metadata: result.fields.get("metadata").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        next_allowed_provisioned_bandwidth_downgrade_time: serde_json::from_value(result.fields.get("nextAllowedProvisionedBandwidthDowngradeTime").cloned().unwrap_or_default())?
            ,
        next_allowed_provisioned_iops_downgrade_time: serde_json::from_value(result.fields.get("nextAllowedProvisionedIopsDowngradeTime").cloned().unwrap_or_default())?
            ,
        next_allowed_quota_downgrade_time: serde_json::from_value(result.fields.get("nextAllowedQuotaDowngradeTime").cloned().unwrap_or_default())?
            ,
        provisioned_bandwidth_mibps: result.fields.get("provisionedBandwidthMibps").cloned().map(serde_json::from_value).transpose()?,
        provisioned_iops: result.fields.get("provisionedIops").cloned().map(serde_json::from_value).transpose()?,
        remaining_retention_days: serde_json::from_value(result.fields.get("remainingRetentionDays").cloned().unwrap_or_default())?
            ,
        root_squash: result.fields.get("rootSquash").cloned().map(serde_json::from_value).transpose()?,
        share_quota: result.fields.get("shareQuota").cloned().map(serde_json::from_value).transpose()?,
        share_usage_bytes: serde_json::from_value(result.fields.get("shareUsageBytes").cloned().unwrap_or_default())?
            ,
        signed_identifiers: result.fields.get("signedIdentifiers").cloned().map(serde_json::from_value).transpose()?,
        snapshot_time: serde_json::from_value(result.fields.get("snapshotTime").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        version: serde_json::from_value(result.fields.get("version").cloned().unwrap_or_default())?
            ,
    })
}
