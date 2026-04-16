use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a deployment.
#[derive(Default)]
pub struct GetDeploymentArgs {
    /// The name of the deployment.
    pub deployment_name: String,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetDeploymentResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The ID of the deployment.
    pub id: String,
    /// the location of the deployment.
    pub location: Option<String>,
    /// The name of the deployment.
    pub name: String,
    /// Deployment properties.
    pub properties: resources::v20210401::DeploymentPropertiesExtendedResponse,
    /// Deployment tags
    pub tags: Option<HashMap<String, String>>,
    /// The type of the deployment.
    pub type_: String,
}

/// Gets a deployment.
pub async fn get_deployment(
    ctx: &Context,
    args: GetDeploymentArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetDeploymentResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("deploymentName".to_string(), json!(args.deployment_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources/v20210401:getDeployment", invoke_args, &opts).await?;

    Ok(GetDeploymentResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
