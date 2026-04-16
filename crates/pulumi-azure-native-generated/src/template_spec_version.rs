use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Template Spec Version object.
/// 
/// Uses Azure REST API version 2022-02-01. In version 2.x of the Azure Native provider, it used API version 2022-02-01.
/// 
/// Other available API versions: 2021-03-01-preview, 2021-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native resources [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct TemplateSpecVersionArgs {
    /// Template Spec version description.
    pub description: Option<Input<String>>,
    /// An array of linked template artifacts.
    pub linked_templates: Option<Vec<Input<resources::LinkedTemplateArtifactArgs>>>,
    /// The location of the Template Spec Version. It must match the location of the parent Template Spec.
    pub location: Option<Input<String>>,
    /// The main Azure Resource Manager template content.
    pub main_template: Option<Input<serde_json::Value>>,
    /// The version metadata. Metadata is an open-ended object and is typically a collection of key-value pairs.
    pub metadata: Option<Input<serde_json::Value>>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Name of the Template Spec.
    pub template_spec_name: Input<String>,
    /// The version of the Template Spec.
    pub template_spec_version: Option<Input<String>>,
    /// The Azure Resource Manager template UI definition content.
    pub ui_form_definition: Option<Input<serde_json::Value>>,
}

/// Template Spec Version object.
/// 
/// Uses Azure REST API version 2022-02-01. In version 2.x of the Azure Native provider, it used API version 2022-02-01.
/// 
/// Other available API versions: 2021-03-01-preview, 2021-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native resources [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct TemplateSpecVersion {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Template Spec version description.
    pub description: Output<serde_json::Value>,
    /// An array of linked template artifacts.
    pub linked_templates: Output<serde_json::Value>,
    /// The location of the Template Spec Version. It must match the location of the parent Template Spec.
    pub location: Output<serde_json::Value>,
    /// The main Azure Resource Manager template content.
    pub main_template: Output<serde_json::Value>,
    /// The version metadata. Metadata is an open-ended object and is typically a collection of key-value pairs.
    pub metadata: Output<serde_json::Value>,
    /// Name of this resource.
    pub name: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Type of this resource.
    pub type_: Output<serde_json::Value>,
    /// The Azure Resource Manager template UI definition content.
    pub ui_form_definition: Output<serde_json::Value>,
}

impl TemplateSpecVersion {
    const TYPE_TOKEN: &'static str = "azure-native:resources:TemplateSpecVersion";

    /// Create a new TemplateSpecVersion resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: TemplateSpecVersionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.linked_templates {
            pulumi_sdk::resolve_input_vec("linkedTemplates", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.main_template {
            pulumi_sdk::resolve_input("mainTemplate", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.metadata {
            pulumi_sdk::resolve_input("metadata", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("templateSpecName", args.template_spec_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.template_spec_version {
            pulumi_sdk::resolve_input("templateSpecVersion", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ui_form_definition {
            pulumi_sdk::resolve_input("uiFormDefinition", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            linked_templates: registered.outputs.get("linkedTemplates")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            main_template: registered.outputs.get("mainTemplate")
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
            ui_form_definition: registered.outputs.get("uiFormDefinition")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
