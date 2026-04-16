use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// .
pub struct BlobContainerLegalHoldArgs {
    /// Name of the Storage Account.
    pub account_name: Input<String>,
    /// When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining legal hold protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted.
    pub allow_protected_append_writes_all: Option<Input<bool>>,
    /// Name of the Blob Container.
    pub container_name: Input<String>,
    /// Name of the resource group that contains the storage account.
    pub resource_group_name: Input<String>,
    /// List of legal hold tags. Each tag should be 3 to 23 alphanumeric characters and is normalized to lower case at SRP.
    pub tags: Vec<Input<String>>,
}

/// .
pub struct BlobContainerLegalHold {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Name of the Storage Account.
    pub account_name: Output<serde_json::Value>,
    /// When enabled, new blocks can be written to both 'Append and Bock Blobs' while maintaining legal hold protection and compliance. Only new blocks can be added and any existing blocks cannot be modified or deleted.
    pub allow_protected_append_writes_all: Output<serde_json::Value>,
    /// Name of the Blob Container.
    pub container_name: Output<serde_json::Value>,
    /// Name of the resource group that contains the storage account.
    pub resource_group_name: Output<serde_json::Value>,
    /// List of legal hold tags. Each tag should be 3 to 23 alphanumeric characters and is normalized to lower case at SRP.
    pub tags: Output<serde_json::Value>,
}

impl BlobContainerLegalHold {
    const TYPE_TOKEN: &'static str = "azure-native:storage:BlobContainerLegalHold";

    /// Create a new BlobContainerLegalHold resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: BlobContainerLegalHoldArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.allow_protected_append_writes_all {
            pulumi_sdk::resolve_input("allowProtectedAppendWritesAll", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("containerName", args.container_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input_vec("tags", args.tags, &mut inputs, &mut deps, &mut prop_deps).await;

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
            account_name: registered.outputs.get("accountName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            allow_protected_append_writes_all: registered.outputs.get("allowProtectedAppendWritesAll")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            container_name: registered.outputs.get("containerName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_group_name: registered.outputs.get("resourceGroupName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
