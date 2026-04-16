use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Properties of the file share, including Id, resource name, resource type, Etag.
/// 
/// Uses Azure REST API version 2024-01-01. In version 2.x of the Azure Native provider, it used API version 2022-09-01.
/// 
/// Other available API versions: 2022-09-01, 2023-01-01, 2023-04-01, 2023-05-01, 2025-01-01, 2025-06-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native storage [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct FileShareArgs {
    /// Access tier for specific share. GpV2 account can choose between TransactionOptimized (default), Hot, and Cool. FileStorage account can choose Premium.
    pub access_tier: Option<Input<serde_json::Value>>,
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Input<String>,
    /// The authentication protocol that is used for the file share. Can only be specified when creating a share.
    pub enabled_protocols: Option<Input<serde_json::Value>>,
    /// Optional, used to expand the properties within share's properties. Valid values are: snapshots. Should be passed as a string with delimiter ','
    pub expand: Option<Input<String>>,
    /// File Share Paid Bursting properties.
    pub file_share_paid_bursting: Option<Input<storage::FileSharePropertiesFileSharePaidBurstingArgs>>,
    /// A name-value pair to associate with the share as metadata.
    pub metadata: Option<HashMap<String, Input<String>>>,
    /// The provisioned bandwidth of the share, in mebibytes per second. This property is only for file shares created under Files Provisioned v2 account type. Please refer to the GetFileServiceUsage API response for the minimum and maximum allowed value for provisioned bandwidth.
    pub provisioned_bandwidth_mibps: Option<Input<i64>>,
    /// The provisioned IOPS of the share. This property is only for file shares created under Files Provisioned v2 account type. Please refer to the GetFileServiceUsage API response for the minimum and maximum allowed value for provisioned IOPS.
    pub provisioned_iops: Option<Input<i64>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// The property is for NFS share only. The default is NoRootSquash.
    pub root_squash: Option<Input<serde_json::Value>>,
    /// The name of the file share within the specified storage account. File share names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub share_name: Option<Input<String>>,
    /// The provisioned size of the share, in gibibytes. Must be greater than 0, and less than or equal to 5TB (5120). For Large File Shares, the maximum size is 102400. For file shares created under Files Provisioned v2 account type, please refer to the GetFileServiceUsage API response for the minimum and maximum allowed provisioned storage size.
    pub share_quota: Option<Input<i64>>,
    /// List of stored access policies specified on the share.
    pub signed_identifiers: Option<Vec<Input<storage::SignedIdentifierArgs>>>,
}

/// Properties of the file share, including Id, resource name, resource type, Etag.
/// 
/// Uses Azure REST API version 2024-01-01. In version 2.x of the Azure Native provider, it used API version 2022-09-01.
/// 
/// Other available API versions: 2022-09-01, 2023-01-01, 2023-04-01, 2023-05-01, 2025-01-01, 2025-06-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native storage [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct FileShare {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Access tier for specific share. GpV2 account can choose between TransactionOptimized (default), Hot, and Cool. FileStorage account can choose Premium.
    pub access_tier: Output<serde_json::Value>,
    /// Indicates the last modification time for share access tier.
    pub access_tier_change_time: Output<serde_json::Value>,
    /// Indicates if there is a pending transition for access tier.
    pub access_tier_status: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Indicates whether the share was deleted.
    pub deleted: Output<serde_json::Value>,
    /// The deleted time if the share was deleted.
    pub deleted_time: Output<serde_json::Value>,
    /// The authentication protocol that is used for the file share. Can only be specified when creating a share.
    pub enabled_protocols: Output<serde_json::Value>,
    /// Resource Etag.
    pub etag: Output<serde_json::Value>,
    /// File Share Paid Bursting properties.
    pub file_share_paid_bursting: Output<serde_json::Value>,
    /// The calculated burst IOPS of the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub included_burst_iops: Output<serde_json::Value>,
    /// Returns the date and time the share was last modified.
    pub last_modified_time: Output<serde_json::Value>,
    /// Specifies whether the lease on a share is of infinite or fixed duration, only when the share is leased.
    pub lease_duration: Output<serde_json::Value>,
    /// Lease state of the share.
    pub lease_state: Output<serde_json::Value>,
    /// The lease status of the share.
    pub lease_status: Output<serde_json::Value>,
    /// The calculated maximum burst credits for the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub max_burst_credits_for_iops: Output<serde_json::Value>,
    /// A name-value pair to associate with the share as metadata.
    pub metadata: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// Returns the next allowed provisioned bandwidth downgrade time for the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub next_allowed_provisioned_bandwidth_downgrade_time: Output<serde_json::Value>,
    /// Returns the next allowed provisioned IOPS downgrade time for the share. This property is only for file shares created under Files Provisioned v2 account type.
    pub next_allowed_provisioned_iops_downgrade_time: Output<serde_json::Value>,
    /// Returns the next allowed provisioned storage size downgrade time for the share. This property is only for file shares created under Files Provisioned v1 SSD and Files Provisioned v2 account type
    pub next_allowed_quota_downgrade_time: Output<serde_json::Value>,
    /// The provisioned bandwidth of the share, in mebibytes per second. This property is only for file shares created under Files Provisioned v2 account type. Please refer to the GetFileServiceUsage API response for the minimum and maximum allowed value for provisioned bandwidth.
    pub provisioned_bandwidth_mibps: Output<serde_json::Value>,
    /// The provisioned IOPS of the share. This property is only for file shares created under Files Provisioned v2 account type. Please refer to the GetFileServiceUsage API response for the minimum and maximum allowed value for provisioned IOPS.
    pub provisioned_iops: Output<serde_json::Value>,
    /// Remaining retention days for share that was soft deleted.
    pub remaining_retention_days: Output<serde_json::Value>,
    /// The property is for NFS share only. The default is NoRootSquash.
    pub root_squash: Output<serde_json::Value>,
    /// The provisioned size of the share, in gibibytes. Must be greater than 0, and less than or equal to 5TB (5120). For Large File Shares, the maximum size is 102400. For file shares created under Files Provisioned v2 account type, please refer to the GetFileServiceUsage API response for the minimum and maximum allowed provisioned storage size.
    pub share_quota: Output<serde_json::Value>,
    /// The approximate size of the data stored on the share. Note that this value may not include all recently created or recently resized files.
    pub share_usage_bytes: Output<serde_json::Value>,
    /// List of stored access policies specified on the share.
    pub signed_identifiers: Output<serde_json::Value>,
    /// Creation time of share snapshot returned in the response of list shares with expand param "snapshots".
    pub snapshot_time: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
    /// The version of the share.
    pub version: Output<serde_json::Value>,
}

impl FileShare {
    const TYPE_TOKEN: &'static str = "azure-native:storage:FileShare";

    /// Create a new FileShare resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: FileShareArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.access_tier {
            pulumi_sdk::resolve_input("accessTier", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.enabled_protocols {
            pulumi_sdk::resolve_input("enabledProtocols", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.expand {
            pulumi_sdk::resolve_input("expand", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.file_share_paid_bursting {
            pulumi_sdk::resolve_input("fileSharePaidBursting", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.metadata {
            pulumi_sdk::resolve_input_map("metadata", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.provisioned_bandwidth_mibps {
            pulumi_sdk::resolve_input("provisionedBandwidthMibps", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.provisioned_iops {
            pulumi_sdk::resolve_input("provisionedIops", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.root_squash {
            pulumi_sdk::resolve_input("rootSquash", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.share_name {
            pulumi_sdk::resolve_input("shareName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.share_quota {
            pulumi_sdk::resolve_input("shareQuota", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.signed_identifiers {
            pulumi_sdk::resolve_input_vec("signedIdentifiers", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            access_tier: registered.outputs.get("accessTier")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            access_tier_change_time: registered.outputs.get("accessTierChangeTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            access_tier_status: registered.outputs.get("accessTierStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deleted: registered.outputs.get("deleted")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deleted_time: registered.outputs.get("deletedTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enabled_protocols: registered.outputs.get("enabledProtocols")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            file_share_paid_bursting: registered.outputs.get("fileSharePaidBursting")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            included_burst_iops: registered.outputs.get("includedBurstIops")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            last_modified_time: registered.outputs.get("lastModifiedTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            lease_duration: registered.outputs.get("leaseDuration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            lease_state: registered.outputs.get("leaseState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            lease_status: registered.outputs.get("leaseStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            max_burst_credits_for_iops: registered.outputs.get("maxBurstCreditsForIops")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            metadata: registered.outputs.get("metadata")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            next_allowed_provisioned_bandwidth_downgrade_time: registered.outputs.get("nextAllowedProvisionedBandwidthDowngradeTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            next_allowed_provisioned_iops_downgrade_time: registered.outputs.get("nextAllowedProvisionedIopsDowngradeTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            next_allowed_quota_downgrade_time: registered.outputs.get("nextAllowedQuotaDowngradeTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioned_bandwidth_mibps: registered.outputs.get("provisionedBandwidthMibps")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioned_iops: registered.outputs.get("provisionedIops")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remaining_retention_days: registered.outputs.get("remainingRetentionDays")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            root_squash: registered.outputs.get("rootSquash")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            share_quota: registered.outputs.get("shareQuota")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            share_usage_bytes: registered.outputs.get("shareUsageBytes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            signed_identifiers: registered.outputs.get("signedIdentifiers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            snapshot_time: registered.outputs.get("snapshotTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            version: registered.outputs.get("version")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
