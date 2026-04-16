use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a resource group.
#[derive(Default)]
pub struct GetResourceGroupArgs {
    /// The name of the resource group to get. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetResourceGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The ID of the resource group.
    pub id: String,
    /// The location of the resource group. It cannot be changed after the resource group has been created. It must be one of the supported Azure locations.
    pub location: String,
    /// The ID of the resource that manages this resource group.
    pub managed_by: Option<String>,
    /// The name of the resource group.
    pub name: String,
    /// The resource group properties.
    pub properties: resources::v20250301::ResourceGroupPropertiesResponse,
    /// The tags attached to the resource group.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the resource group.
    pub type_: String,
}

/// Gets a resource group.
pub async fn get_resource_group(
    ctx: &Context,
    args: GetResourceGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetResourceGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources/v20250301:getResourceGroup", invoke_args, &opts).await?;

    Ok(GetResourceGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        managed_by: result.fields.get("managedBy").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
