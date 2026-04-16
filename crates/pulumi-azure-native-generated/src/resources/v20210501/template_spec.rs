use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Template Spec object.
pub struct TemplateSpecArgs {
    /// Template Spec description.
    pub description: Option<Input<String>>,
    /// Template Spec display name.
    pub display_name: Option<Input<String>>,
    /// The location of the Template Spec. It cannot be changed after Template Spec creation. It must be one of the supported Azure locations.
    pub location: Option<Input<String>>,
    /// The Template Spec metadata. Metadata is an open-ended object and is typically a collection of key-value pairs.
    pub metadata: Option<Input<serde_json::Value>>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Name of the Template Spec.
    pub template_spec_name: Option<Input<String>>,
}

/// Template Spec object.
pub struct TemplateSpec {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Template Spec description.
    pub description: Output<serde_json::Value>,
    /// Template Spec display name.
    pub display_name: Output<serde_json::Value>,
    /// The location of the Template Spec. It cannot be changed after Template Spec creation. It must be one of the supported Azure locations.
    pub location: Output<serde_json::Value>,
    /// The Template Spec metadata. Metadata is an open-ended object and is typically a collection of key-value pairs.
    pub metadata: Output<serde_json::Value>,
    /// Name of this resource.
    pub name: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Type of this resource.
    pub type_: Output<serde_json::Value>,
    /// High-level information about the versions within this Template Spec. The keys are the version names. Only populated if the $expand query parameter is set to 'versions'.
    pub versions: Output<serde_json::Value>,
}

impl TemplateSpec {
    const TYPE_TOKEN: &'static str = "azure-native:resources/v20210501:TemplateSpec";

    /// Create a new TemplateSpec resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: TemplateSpecArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.display_name {
            pulumi_sdk::resolve_input("displayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.metadata {
            pulumi_sdk::resolve_input("metadata", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.template_spec_name {
            pulumi_sdk::resolve_input("templateSpecName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            display_name: registered.outputs.get("displayName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            metadata: registered.outputs.get("metadata")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            versions: registered.outputs.get("versions")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
