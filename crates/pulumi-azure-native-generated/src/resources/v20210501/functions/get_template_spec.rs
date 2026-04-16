use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a Template Spec with a given name.
#[derive(Default)]
pub struct GetTemplateSpecArgs {
    /// Allows for expansion of additional Template Spec details in the response. Optional.
    pub expand: Option<String>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
    /// Name of the Template Spec.
    pub template_spec_name: String,
}

/// Result of the function invocation.
pub struct GetTemplateSpecResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Template Spec description.
    pub description: Option<String>,
    /// Template Spec display name.
    pub display_name: Option<String>,
    /// String Id used to locate any resource on Azure.
    pub id: String,
    /// The location of the Template Spec. It cannot be changed after Template Spec creation. It must be one of the supported Azure locations.
    pub location: String,
    /// The Template Spec metadata. Metadata is an open-ended object and is typically a collection of key-value pairs.
    pub metadata: Option<serde_json::Value>,
    /// Name of this resource.
    pub name: String,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: resources::v20210501::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Type of this resource.
    pub type_: String,
    /// High-level information about the versions within this Template Spec. The keys are the version names. Only populated if the $expand query parameter is set to 'versions'.
    pub versions: HashMap<String, resources::v20210501::TemplateSpecVersionInfoResponse>,
}

/// Gets a Template Spec with a given name.
pub async fn get_template_spec(
    ctx: &Context,
    args: GetTemplateSpecArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetTemplateSpecResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("templateSpecName".to_string(), json!(args.template_spec_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources/v20210501:getTemplateSpec", invoke_args, &opts).await?;

    Ok(GetTemplateSpecResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        display_name: result.fields.get("displayName").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        metadata: result.fields.get("metadata").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        versions: serde_json::from_value(result.fields.get("versions").cloned().unwrap_or_default())?
            ,
    })
}
