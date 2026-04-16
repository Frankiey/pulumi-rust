use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the Deployment stack with the given name.
/// 
/// Uses Azure REST API version 2025-07-01.
#[derive(Default)]
pub struct GetDeploymentStacksWhatIfResultsAtResourceGroupArgs {
    /// Name of the deployment stack what-if result.
    pub deployment_stacks_what_if_result_name: String,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetDeploymentStacksWhatIfResultsAtResourceGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The geo-location where the resource lives. Required for subscription and management group scoped stacks. The location is inherited from the resource group for resource group scoped stacks.
    pub location: Option<String>,
    /// The name of the resource
    pub name: String,
    /// The resource-specific properties for this resource.
    pub properties: resources::DeploymentStacksWhatIfResultPropertiesResponse,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: resources::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the Deployment stack with the given name.
pub async fn get_deployment_stacks_what_if_results_at_resource_group(
    ctx: &Context,
    args: GetDeploymentStacksWhatIfResultsAtResourceGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetDeploymentStacksWhatIfResultsAtResourceGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("deploymentStacksWhatIfResultName".to_string(), json!(args.deployment_stacks_what_if_result_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources:getDeploymentStacksWhatIfResultsAtResourceGroup", invoke_args, &opts).await?;

    Ok(GetDeploymentStacksWhatIfResultsAtResourceGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
