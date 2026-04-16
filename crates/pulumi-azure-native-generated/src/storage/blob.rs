use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Manages a Blob within a Storage Container. For the supported combinations of properties and features please see [here](https://learn.microsoft.com/en-us/azure/storage/blobs/storage-feature-support-in-storage-accounts).
pub struct BlobArgs {
    /// The access tier of the storage blob. Only supported for standard storage accounts, not premium.
    pub access_tier: Option<Input<storage::BlobAccessTierArgs>>,
    /// Specifies the storage account in which to create the storage container.
    pub account_name: Input<String>,
    /// The name of the storage blob. Must be unique within the storage container the blob is located. If this property is not specified it will be set to the name of the resource.
    pub blob_name: Option<Input<String>>,
    /// The name of the storage container in which this blob should be created.
    pub container_name: Input<String>,
    /// The MD5 sum of the blob contents, base64-encoded. Cannot be defined if blob type is Append.
    pub content_md5: Option<Input<String>>,
    /// The content type of the storage blob. Defaults to `application/octet-stream`.
    pub content_type: Option<Input<String>>,
    /// A map of custom blob metadata.
    pub metadata: Option<HashMap<String, Input<String>>>,
    /// The name of the resource group within the user's subscription.
    pub resource_group_name: Input<String>,
    /// An asset to copy to the blob contents. This field cannot be specified for Append blobs.
    pub source: Option<Input<AssetOrArchive>>,
    /// The type of the storage blob to be created. Defaults to 'Block'.
    pub type_: Option<Input<storage::BlobTypeArgs>>,
}

/// Manages a Blob within a Storage Container. For the supported combinations of properties and features please see [here](https://learn.microsoft.com/en-us/azure/storage/blobs/storage-feature-support-in-storage-accounts).
pub struct Blob {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The access tier of the storage blob. Only supported for standard storage accounts, not premium.
    pub access_tier: Output<serde_json::Value>,
    /// The MD5 sum of the blob contents.
    pub content_md5: Output<serde_json::Value>,
    /// The content type of the storage blob.
    pub content_type: Output<serde_json::Value>,
    /// A map of custom blob metadata.
    pub metadata: Output<serde_json::Value>,
    /// The name of the storage blob.
    pub name: Output<serde_json::Value>,
    /// The type of the storage blob to be created.
    pub type_: Output<serde_json::Value>,
    /// The URL of the blob.
    pub url: Output<serde_json::Value>,
}

impl Blob {
    const TYPE_TOKEN: &'static str = "azure-native:storage:Blob";

    /// Create a new Blob resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: BlobArgs,
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
        if let Some(v) = args.blob_name {
            pulumi_sdk::resolve_input("blobName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("containerName", args.container_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.content_md5 {
            pulumi_sdk::resolve_input("contentMd5", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.content_type {
            pulumi_sdk::resolve_input("contentType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.metadata {
            pulumi_sdk::resolve_input_map("metadata", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.source {
            pulumi_sdk::resolve_input("source", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.type_ {
            pulumi_sdk::resolve_input("type", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            content_md5: registered.outputs.get("contentMd5")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            content_type: registered.outputs.get("contentType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            metadata: registered.outputs.get("metadata")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            url: registered.outputs.get("url")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
