use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Wrapper resource for tags API requests and responses.
#[derive(Default)]
pub struct GetTagAtScopeArgs {
    /// The resource scope.
    pub scope: String,
}

/// Result of the function invocation.
pub struct GetTagAtScopeResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The ID of the tags wrapper resource.
    pub id: String,
    /// The name of the tags wrapper resource.
    pub name: String,
    /// The set of tags.
    pub properties: resources::v20240701::TagsResponse,
    /// The type of the tags wrapper resource.
    pub type_: String,
}

/// Wrapper resource for tags API requests and responses.
pub async fn get_tag_at_scope(
    ctx: &Context,
    args: GetTagAtScopeArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetTagAtScopeResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("scope".to_string(), json!(args.scope));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources/v20240701:getTagAtScope", invoke_args, &opts).await?;

    Ok(GetTagAtScopeResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
