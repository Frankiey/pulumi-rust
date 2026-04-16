use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Properties of the blob container, including Id, resource name, resource type, Etag.
pub struct BlobContainerArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Input<String>,
    /// The name of the blob container within the specified storage account. Blob container names must be between 3 and 63 characters in length and use numbers, lower-case letters and dash (-) only. Every dash (-) character must be immediately preceded and followed by a letter or number.
    pub container_name: Option<Input<String>>,
    /// Default the container to use specified encryption scope for all writes.
    pub default_encryption_scope: Option<Input<String>>,
    /// Block override of encryption scope from the container default.
    pub deny_encryption_scope_override: Option<Input<bool>>,
    /// Enable NFSv3 all squash on blob container.
    pub enable_nfs_v3all_squash: Option<Input<bool>>,
    /// Enable NFSv3 root squash on blob container.
    pub enable_nfs_v3root_squash: Option<Input<bool>>,
    /// The object level immutability property of the container. The property is immutable and can only be set to true at the container creation time. Existing containers must undergo a migration process.
    pub immutable_storage_with_versioning: Option<Input<storage::v20240101::ImmutableStorageWithVersioningArgs>>,
    /// A name-value pair to associate with the container as metadata.
    pub metadata: Option<HashMap<String, Input<String>>>,
    /// Specifies whether data in the container may be accessed publicly and the level of access.
    pub public_access: Option<Input<storage::v20240101::PublicAccessArgs>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
}

/// Properties of the blob container, including Id, resource name, resource type, Etag.
pub struct BlobContainer {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Default the container to use specified encryption scope for all writes.
    pub default_encryption_scope: Output<serde_json::Value>,
    /// Indicates whether the blob container was deleted.
    pub deleted: Output<serde_json::Value>,
    /// Blob container deletion time.
    pub deleted_time: Output<serde_json::Value>,
    /// Block override of encryption scope from the container default.
    pub deny_encryption_scope_override: Output<serde_json::Value>,
    /// Enable NFSv3 all squash on blob container.
    pub enable_nfs_v3all_squash: Output<serde_json::Value>,
    /// Enable NFSv3 root squash on blob container.
    pub enable_nfs_v3root_squash: Output<serde_json::Value>,
    /// Resource Etag.
    pub etag: Output<serde_json::Value>,
    /// The hasImmutabilityPolicy public property is set to true by SRP if ImmutabilityPolicy has been created for this container. The hasImmutabilityPolicy public property is set to false by SRP if ImmutabilityPolicy has not been created for this container.
    pub has_immutability_policy: Output<serde_json::Value>,
    /// The hasLegalHold public property is set to true by SRP if there are at least one existing tag. The hasLegalHold public property is set to false by SRP if all existing legal hold tags are cleared out. There can be a maximum of 1000 blob containers with hasLegalHold=true for a given account.
    pub has_legal_hold: Output<serde_json::Value>,
    /// The ImmutabilityPolicy property of the container.
    pub immutability_policy: Output<serde_json::Value>,
    /// The object level immutability property of the container. The property is immutable and can only be set to true at the container creation time. Existing containers must undergo a migration process.
    pub immutable_storage_with_versioning: Output<serde_json::Value>,
    /// Returns the date and time the container was last modified.
    pub last_modified_time: Output<serde_json::Value>,
    /// Specifies whether the lease on a container is of infinite or fixed duration, only when the container is leased.
    pub lease_duration: Output<serde_json::Value>,
    /// Lease state of the container.
    pub lease_state: Output<serde_json::Value>,
    /// The lease status of the container.
    pub lease_status: Output<serde_json::Value>,
    /// The LegalHold property of the container.
    pub legal_hold: Output<serde_json::Value>,
    /// A name-value pair to associate with the container as metadata.
    pub metadata: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// Specifies whether data in the container may be accessed publicly and the level of access.
    pub public_access: Output<serde_json::Value>,
    /// Remaining retention days for soft deleted blob container.
    pub remaining_retention_days: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
    /// The version of the deleted blob container.
    pub version: Output<serde_json::Value>,
}

impl BlobContainer {
    const TYPE_TOKEN: &'static str = "azure-native:storage/v20240101:BlobContainer";

    /// Create a new BlobContainer resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: BlobContainerArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.container_name {
            pulumi_sdk::resolve_input("containerName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.default_encryption_scope {
            pulumi_sdk::resolve_input("defaultEncryptionScope", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.deny_encryption_scope_override {
            pulumi_sdk::resolve_input("denyEncryptionScopeOverride", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_nfs_v3all_squash {
            pulumi_sdk::resolve_input("enableNfsV3AllSquash", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_nfs_v3root_squash {
            pulumi_sdk::resolve_input("enableNfsV3RootSquash", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.immutable_storage_with_versioning {
            pulumi_sdk::resolve_input("immutableStorageWithVersioning", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.metadata {
            pulumi_sdk::resolve_input_map("metadata", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_access {
            pulumi_sdk::resolve_input("publicAccess", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            default_encryption_scope: registered.outputs.get("defaultEncryptionScope")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deleted: registered.outputs.get("deleted")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deleted_time: registered.outputs.get("deletedTime")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deny_encryption_scope_override: registered.outputs.get("denyEncryptionScopeOverride")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_nfs_v3all_squash: registered.outputs.get("enableNfsV3AllSquash")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_nfs_v3root_squash: registered.outputs.get("enableNfsV3RootSquash")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            has_immutability_policy: registered.outputs.get("hasImmutabilityPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            has_legal_hold: registered.outputs.get("hasLegalHold")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            immutability_policy: registered.outputs.get("immutabilityPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            immutable_storage_with_versioning: registered.outputs.get("immutableStorageWithVersioning")
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
            legal_hold: registered.outputs.get("legalHold")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            metadata: registered.outputs.get("metadata")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_access: registered.outputs.get("publicAccess")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remaining_retention_days: registered.outputs.get("remainingRetentionDays")
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
