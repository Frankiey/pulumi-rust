use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a resource.
#[derive(Default)]
pub struct GetResourceArgs {
    /// The API version to use for the operation.
    pub api_version: String,
    /// The parent resource identity.
    pub parent_resource_path: String,
    /// The name of the resource group containing the resource to get. The name is case insensitive.
    pub resource_group_name: String,
    /// The name of the resource to get.
    pub resource_name: String,
    /// The namespace of the resource provider.
    pub resource_provider_namespace: String,
    /// The resource type of the resource.
    pub resource_type: String,
}

/// Result of the function invocation.
pub struct GetResourceResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Resource extended location.
    pub extended_location: Option<resources::v20240301::ExtendedLocationResponse>,
    /// Resource ID
    pub id: String,
    /// The identity of the resource.
    pub identity: Option<resources::v20240301::IdentityResponse>,
    /// The kind of the resource.
    pub kind: Option<String>,
    /// Resource location
    pub location: Option<String>,
    /// ID of the resource that manages this resource.
    pub managed_by: Option<String>,
    /// Resource name
    pub name: String,
    /// The plan of the resource.
    pub plan: Option<resources::v20240301::PlanResponse>,
    /// The resource properties.
    pub properties: serde_json::Value,
    /// The SKU of the resource.
    pub sku: Option<resources::v20240301::SkuResponse>,
    /// Resource tags
    pub tags: Option<HashMap<String, String>>,
    /// Resource type
    pub type_: String,
}

/// Gets a resource.
pub async fn get_resource(
    ctx: &Context,
    args: GetResourceArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetResourceResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("apiVersion".to_string(), json!(args.api_version));
    invoke_args.insert("parentResourcePath".to_string(), json!(args.parent_resource_path));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("resourceName".to_string(), json!(args.resource_name));
    invoke_args.insert("resourceProviderNamespace".to_string(), json!(args.resource_provider_namespace));
    invoke_args.insert("resourceType".to_string(), json!(args.resource_type));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources/v20240301:getResource", invoke_args, &opts).await?;

    Ok(GetResourceResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        extended_location: result.fields.get("extendedLocation").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        kind: result.fields.get("kind").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        managed_by: result.fields.get("managedBy").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        plan: result.fields.get("plan").cloned().map(serde_json::from_value).transpose()?,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
