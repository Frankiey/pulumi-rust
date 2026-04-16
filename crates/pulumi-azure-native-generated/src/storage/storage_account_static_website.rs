use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Enables the static website feature of a storage account.
pub struct StorageAccountStaticWebsiteArgs {
    /// The name of the storage account within the specified resource group.
    pub account_name: Input<String>,
    /// The absolute path to a custom webpage that should be used when a request is made which does not correspond to an existing file.
    pub error404document: Option<Input<String>>,
    /// The webpage that Azure Storage serves for requests to the root of a website or any sub-folder. For example, 'index.html'. The value is case-sensitive.
    pub index_document: Option<Input<String>>,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: Input<String>,
}

/// Enables the static website feature of a storage account.
pub struct StorageAccountStaticWebsite {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The name of the container to upload blobs to.
    pub container_name: Output<serde_json::Value>,
    /// The absolute path to a custom webpage that should be used when a request is made which does not correspond to an existing file.
    pub error404document: Output<serde_json::Value>,
    /// The webpage that Azure Storage serves for requests to the root of a website or any sub-folder. For example, 'index.html'. The value is case-sensitive.
    pub index_document: Output<serde_json::Value>,
}

impl StorageAccountStaticWebsite {
    const TYPE_TOKEN: &'static str = "azure-native:storage:StorageAccountStaticWebsite";

    /// Create a new StorageAccountStaticWebsite resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: StorageAccountStaticWebsiteArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("accountName", args.account_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.error404document {
            pulumi_sdk::resolve_input("error404Document", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.index_document {
            pulumi_sdk::resolve_input("indexDocument", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            container_name: registered.outputs.get("containerName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            error404document: registered.outputs.get("error404Document")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            index_document: registered.outputs.get("indexDocument")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
