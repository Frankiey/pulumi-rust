use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The properties of a storage account’s Blob service.
pub struct BlobServicePropertiesArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: Input<String>,
    /// Deprecated in favor of isVersioningEnabled property.
    pub automatic_snapshot_policy_enabled: Option<Input<bool>>,
    /// The name of the blob Service within the specified storage account. Blob Service Name must be 'default'
    pub blob_services_name: Option<Input<String>>,
    /// The blob service properties for change feed events.
    pub change_feed: Option<Input<storage::v20240101::ChangeFeedArgs>>,
    /// The blob service properties for container soft delete.
    pub container_delete_retention_policy: Option<Input<storage::v20240101::DeleteRetentionPolicyArgs>>,
    /// Specifies CORS rules for the Blob service. You can include up to five CorsRule elements in the request. If no CorsRule elements are included in the request body, all CORS rules will be deleted, and CORS will be disabled for the Blob service.
    pub cors: Option<Input<storage::v20240101::CorsRulesArgs>>,
    /// DefaultServiceVersion indicates the default version to use for requests to the Blob service if an incoming request’s version is not specified. Possible values include version 2008-10-27 and all more recent versions.
    pub default_service_version: Option<Input<String>>,
    /// The blob service properties for blob soft delete.
    pub delete_retention_policy: Option<Input<storage::v20240101::DeleteRetentionPolicyArgs>>,
    /// Versioning is enabled if set to true.
    pub is_versioning_enabled: Option<Input<bool>>,
    /// The blob service property to configure last access time based tracking policy.
    pub last_access_time_tracking_policy: Option<Input<storage::v20240101::LastAccessTimeTrackingPolicyArgs>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// The blob service properties for blob restore policy.
    pub restore_policy: Option<Input<storage::v20240101::RestorePolicyPropertiesArgs>>,
}

/// The properties of a storage account’s Blob service.
pub struct BlobServiceProperties {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Deprecated in favor of isVersioningEnabled property.
    pub automatic_snapshot_policy_enabled: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The blob service properties for change feed events.
    pub change_feed: Output<serde_json::Value>,
    /// The blob service properties for container soft delete.
    pub container_delete_retention_policy: Output<serde_json::Value>,
    /// Specifies CORS rules for the Blob service. You can include up to five CorsRule elements in the request. If no CorsRule elements are included in the request body, all CORS rules will be deleted, and CORS will be disabled for the Blob service.
    pub cors: Output<serde_json::Value>,
    /// DefaultServiceVersion indicates the default version to use for requests to the Blob service if an incoming request’s version is not specified. Possible values include version 2008-10-27 and all more recent versions.
    pub default_service_version: Output<serde_json::Value>,
    /// The blob service properties for blob soft delete.
    pub delete_retention_policy: Output<serde_json::Value>,
    /// Versioning is enabled if set to true.
    pub is_versioning_enabled: Output<serde_json::Value>,
    /// The blob service property to configure last access time based tracking policy.
    pub last_access_time_tracking_policy: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// The blob service properties for blob restore policy.
    pub restore_policy: Output<serde_json::Value>,
    /// Sku name and tier.
    pub sku: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl BlobServiceProperties {
    const TYPE_TOKEN: &'static str = "azure-native:storage/v20240101:BlobServiceProperties";

    /// Create a new BlobServiceProperties resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: BlobServicePropertiesArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.automatic_snapshot_policy_enabled {
            pulumi_sdk::resolve_input("automaticSnapshotPolicyEnabled", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.blob_services_name {
            pulumi_sdk::resolve_input("blobServicesName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.change_feed {
            pulumi_sdk::resolve_input("changeFeed", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.container_delete_retention_policy {
            pulumi_sdk::resolve_input("containerDeleteRetentionPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.cors {
            pulumi_sdk::resolve_input("cors", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.default_service_version {
            pulumi_sdk::resolve_input("defaultServiceVersion", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.delete_retention_policy {
            pulumi_sdk::resolve_input("deleteRetentionPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_versioning_enabled {
            pulumi_sdk::resolve_input("isVersioningEnabled", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.last_access_time_tracking_policy {
            pulumi_sdk::resolve_input("lastAccessTimeTrackingPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.restore_policy {
            pulumi_sdk::resolve_input("restorePolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            automatic_snapshot_policy_enabled: registered.outputs.get("automaticSnapshotPolicyEnabled")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            change_feed: registered.outputs.get("changeFeed")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            container_delete_retention_policy: registered.outputs.get("containerDeleteRetentionPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            cors: registered.outputs.get("cors")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            default_service_version: registered.outputs.get("defaultServiceVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            delete_retention_policy: registered.outputs.get("deleteRetentionPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            is_versioning_enabled: registered.outputs.get("isVersioningEnabled")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            last_access_time_tracking_policy: registered.outputs.get("lastAccessTimeTrackingPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            restore_policy: registered.outputs.get("restorePolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
