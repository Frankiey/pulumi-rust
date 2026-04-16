use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the Deployment stack with the given name.
/// 
/// Uses Azure REST API version 2024-03-01.
#[derive(Default)]
pub struct GetDeploymentStackAtResourceGroupArgs {
    /// Name of the deployment stack.
    pub deployment_stack_name: String,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetDeploymentStackAtResourceGroupResult {
    /// Defines the behavior of resources that are no longer managed after the Deployment stack is updated or deleted.
    pub action_on_unmanage: resources::ActionOnUnmanageResponse,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The correlation id of the last Deployment stack upsert or delete operation. It is in GUID format and is used for tracing.
    pub correlation_id: String,
    /// The debug setting of the deployment.
    pub debug_setting: Option<resources::DeploymentStacksDebugSettingResponse>,
    /// An array of resources that were deleted during the most recent Deployment stack update. Deleted means that the resource was removed from the template and relevant deletion operations were specified.
    pub deleted_resources: Vec<resources::ResourceReferenceResponse>,
    /// Defines how resources deployed by the stack are locked.
    pub deny_settings: resources::DenySettingsResponse,
    /// The resourceId of the deployment resource created by the deployment stack.
    pub deployment_id: String,
    /// The scope at which the initial deployment should be created. If a scope is not specified, it will default to the scope of the deployment stack. Valid scopes are: management group (format: '/providers/Microsoft.Management/managementGroups/{managementGroupId}'), subscription (format: '/subscriptions/{subscriptionId}'), resource group (format: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}').
    pub deployment_scope: Option<String>,
    /// Deployment stack description. Max length of 4096 characters.
    pub description: Option<String>,
    /// An array of resources that were detached during the most recent Deployment stack update. Detached means that the resource was removed from the template, but no relevant deletion operations were specified. So, the resource still exists while no longer being associated with the stack.
    pub detached_resources: Vec<resources::ResourceReferenceResponse>,
    /// The duration of the last successful Deployment stack update.
    pub duration: String,
    /// The error detail.
    pub error: resources::ErrorDetailResponse,
    /// An array of resources that failed to reach goal state during the most recent update. Each resourceId is accompanied by an error message.
    pub failed_resources: Vec<resources::ResourceReferenceExtendedResponse>,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The geo-location where the resource lives. Required for subscription and management group scoped stacks. The location is inherited from the resource group for resource group scoped stacks.
    pub location: Option<String>,
    /// The name of the resource
    pub name: String,
    /// The outputs of the deployment resource created by the deployment stack.
    pub outputs: serde_json::Value,
    /// Name and value pairs that define the deployment parameters for the template. Use this element when providing the parameter values directly in the request, rather than linking to an existing parameter file. Use either the parametersLink property or the parameters property, but not both.
    pub parameters: Option<HashMap<String, resources::DeploymentParameterResponse>>,
    /// The URI of parameters file. Use this element to link to an existing parameters file. Use either the parametersLink property or the parameters property, but not both.
    pub parameters_link: Option<resources::DeploymentStacksParametersLinkResponse>,
    /// State of the deployment stack.
    pub provisioning_state: String,
    /// An array of resources currently managed by the deployment stack.
    pub resources: Vec<resources::ManagedResourceReferenceResponse>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: resources::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the Deployment stack with the given name.
pub async fn get_deployment_stack_at_resource_group(
    ctx: &Context,
    args: GetDeploymentStackAtResourceGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetDeploymentStackAtResourceGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("deploymentStackName".to_string(), json!(args.deployment_stack_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources:getDeploymentStackAtResourceGroup", invoke_args, &opts).await?;

    Ok(GetDeploymentStackAtResourceGroupResult {
        action_on_unmanage: serde_json::from_value(result.fields.get("actionOnUnmanage").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        correlation_id: serde_json::from_value(result.fields.get("correlationId").cloned().unwrap_or_default())?
            ,
        debug_setting: result.fields.get("debugSetting").cloned().map(serde_json::from_value).transpose()?,
        deleted_resources: serde_json::from_value(result.fields.get("deletedResources").cloned().unwrap_or_default())?
            ,
        deny_settings: serde_json::from_value(result.fields.get("denySettings").cloned().unwrap_or_default())?
            ,
        deployment_id: serde_json::from_value(result.fields.get("deploymentId").cloned().unwrap_or_default())?
            ,
        deployment_scope: result.fields.get("deploymentScope").cloned().map(serde_json::from_value).transpose()?,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        detached_resources: serde_json::from_value(result.fields.get("detachedResources").cloned().unwrap_or_default())?
            ,
        duration: serde_json::from_value(result.fields.get("duration").cloned().unwrap_or_default())?
            ,
        error: serde_json::from_value(result.fields.get("error").cloned().unwrap_or_default())?
            ,
        failed_resources: serde_json::from_value(result.fields.get("failedResources").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        outputs: serde_json::from_value(result.fields.get("outputs").cloned().unwrap_or_default())?
            ,
        parameters: result.fields.get("parameters").cloned().map(serde_json::from_value).transpose()?,
        parameters_link: result.fields.get("parametersLink").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resources: serde_json::from_value(result.fields.get("resources").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
