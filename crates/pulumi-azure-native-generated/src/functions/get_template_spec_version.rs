use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a Template Spec version from a specific Template Spec.
/// 
/// Uses Azure REST API version 2022-02-01.
#[derive(Default)]
pub struct GetTemplateSpecVersionArgs {
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
    /// Name of the Template Spec.
    pub template_spec_name: String,
    /// The version of the Template Spec.
    pub template_spec_version: String,
}

/// Result of the function invocation.
pub struct GetTemplateSpecVersionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Template Spec version description.
    pub description: Option<String>,
    /// String Id used to locate any resource on Azure.
    pub id: String,
    /// An array of linked template artifacts.
    pub linked_templates: Option<Vec<resources::LinkedTemplateArtifactResponse>>,
    /// The location of the Template Spec Version. It must match the location of the parent Template Spec.
    pub location: String,
    /// The main Azure Resource Manager template content.
    pub main_template: Option<serde_json::Value>,
    /// The version metadata. Metadata is an open-ended object and is typically a collection of key-value pairs.
    pub metadata: Option<serde_json::Value>,
    /// Name of this resource.
    pub name: String,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: resources::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Type of this resource.
    pub type_: String,
    /// The Azure Resource Manager template UI definition content.
    pub ui_form_definition: Option<serde_json::Value>,
}

/// Gets a Template Spec version from a specific Template Spec.
pub async fn get_template_spec_version(
    ctx: &Context,
    args: GetTemplateSpecVersionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetTemplateSpecVersionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("templateSpecName".to_string(), json!(args.template_spec_name));
    invoke_args.insert("templateSpecVersion".to_string(), json!(args.template_spec_version));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources:getTemplateSpecVersion", invoke_args, &opts).await?;

    Ok(GetTemplateSpecVersionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        linked_templates: result.fields.get("linkedTemplates").cloned().map(serde_json::from_value).transpose()?,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        main_template: result.fields.get("mainTemplate").cloned().map(serde_json::from_value).transpose()?,
        metadata: result.fields.get("metadata").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        ui_form_definition: result.fields.get("uiFormDefinition").cloned().map(serde_json::from_value).transpose()?,
    })
}
