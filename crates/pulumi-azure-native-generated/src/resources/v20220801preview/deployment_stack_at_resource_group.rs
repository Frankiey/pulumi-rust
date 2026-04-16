use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Deployment stack object.
pub struct DeploymentStackAtResourceGroupArgs {
    /// Defines the behavior of resources that are no longer managed after the Deployment stack is updated or deleted.
    pub action_on_unmanage: Input<resources::v20220801preview::ActionOnUnmanageArgs>,
    /// The debug setting of the deployment.
    pub debug_setting: Option<Input<resources::v20220801preview::DeploymentStacksDebugSettingArgs>>,
    /// Defines how resources deployed by the stack are locked.
    pub deny_settings: Input<resources::v20220801preview::DenySettingsArgs>,
    /// The scope at which the initial deployment should be created. If a scope is not specified, it will default to the scope of the deployment stack. Valid scopes are: management group (format: '/providers/Microsoft.Management/managementGroups/{managementGroupId}'), subscription (format: '/subscriptions/{subscriptionId}'), resource group (format: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}').
    pub deployment_scope: Option<Input<String>>,
    /// Name of the deployment stack.
    pub deployment_stack_name: Option<Input<String>>,
    /// Deployment stack description. Max length of 4096 characters.
    pub description: Option<Input<String>>,
    /// The geo-location where the resource lives. Required for subscription and management group scoped stacks. The location is inherited from the resource group for resource group scoped stacks.
    pub location: Option<Input<String>>,
    /// Name and value pairs that define the deployment parameters for the template. Use this element when providing the parameter values directly in the request, rather than linking to an existing parameter file. Use either the parametersLink property or the parameters property, but not both.
    pub parameters: Option<HashMap<String, Input<resources::v20220801preview::DeploymentParameterArgs>>>,
    /// The URI of parameters file. Use this element to link to an existing parameters file. Use either the parametersLink property or the parameters property, but not both.
    pub parameters_link: Option<Input<resources::v20220801preview::DeploymentStacksParametersLinkArgs>>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The template content. You use this element when you want to pass the template syntax directly in the request rather than link to an existing template. It can be a JObject or well-formed JSON string. Use either the templateLink property or the template property, but not both.
    pub template: Option<Input<serde_json::Value>>,
    /// The URI of the template. Use either the templateLink property or the template property, but not both.
    pub template_link: Option<Input<resources::v20220801preview::DeploymentStacksTemplateLinkArgs>>,
}

/// Deployment stack object.
pub struct DeploymentStackAtResourceGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Defines the behavior of resources that are no longer managed after the Deployment stack is updated or deleted.
    pub action_on_unmanage: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The debug setting of the deployment.
    pub debug_setting: Output<serde_json::Value>,
    /// An array of resources that were deleted during the most recent Deployment stack update. Deleted means that the resource was removed from the template and relevant deletion operations were specified.
    pub deleted_resources: Output<serde_json::Value>,
    /// Defines how resources deployed by the stack are locked.
    pub deny_settings: Output<serde_json::Value>,
    /// The resourceId of the deployment resource created by the deployment stack.
    pub deployment_id: Output<serde_json::Value>,
    /// The scope at which the initial deployment should be created. If a scope is not specified, it will default to the scope of the deployment stack. Valid scopes are: management group (format: '/providers/Microsoft.Management/managementGroups/{managementGroupId}'), subscription (format: '/subscriptions/{subscriptionId}'), resource group (format: '/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}').
    pub deployment_scope: Output<serde_json::Value>,
    /// Deployment stack description. Max length of 4096 characters.
    pub description: Output<serde_json::Value>,
    /// An array of resources that were detached during the most recent Deployment stack update. Detached means that the resource was removed from the template, but no relevant deletion operations were specified. So, the resource still exists while no longer being associated with the stack.
    pub detached_resources: Output<serde_json::Value>,
    /// The duration of the last successful Deployment stack update.
    pub duration: Output<serde_json::Value>,
    /// The error detail.
    pub error: Output<serde_json::Value>,
    /// An array of resources that failed to reach goal state during the most recent update. Each resourceId is accompanied by an error message.
    pub failed_resources: Output<serde_json::Value>,
    /// The geo-location where the resource lives. Required for subscription and management group scoped stacks. The location is inherited from the resource group for resource group scoped stacks.
    pub location: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// The outputs of the deployment resource created by the deployment stack.
    pub outputs: Output<serde_json::Value>,
    /// Name and value pairs that define the deployment parameters for the template. Use this element when providing the parameter values directly in the request, rather than linking to an existing parameter file. Use either the parametersLink property or the parameters property, but not both.
    pub parameters: Output<serde_json::Value>,
    /// The URI of parameters file. Use this element to link to an existing parameters file. Use either the parametersLink property or the parameters property, but not both.
    pub parameters_link: Output<serde_json::Value>,
    /// State of the deployment stack.
    pub provisioning_state: Output<serde_json::Value>,
    /// An array of resources currently managed by the deployment stack.
    pub resources: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl DeploymentStackAtResourceGroup {
    const TYPE_TOKEN: &'static str = "azure-native:resources/v20220801preview:DeploymentStackAtResourceGroup";

    /// Create a new DeploymentStackAtResourceGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: DeploymentStackAtResourceGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("actionOnUnmanage", args.action_on_unmanage, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.debug_setting {
            pulumi_sdk::resolve_input("debugSetting", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("denySettings", args.deny_settings, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.deployment_scope {
            pulumi_sdk::resolve_input("deploymentScope", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.deployment_stack_name {
            pulumi_sdk::resolve_input("deploymentStackName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.parameters {
            pulumi_sdk::resolve_input_map("parameters", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.parameters_link {
            pulumi_sdk::resolve_input("parametersLink", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.template {
            pulumi_sdk::resolve_input("template", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.template_link {
            pulumi_sdk::resolve_input("templateLink", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            action_on_unmanage: registered.outputs.get("actionOnUnmanage")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            debug_setting: registered.outputs.get("debugSetting")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deleted_resources: registered.outputs.get("deletedResources")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deny_settings: registered.outputs.get("denySettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deployment_id: registered.outputs.get("deploymentId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            deployment_scope: registered.outputs.get("deploymentScope")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            detached_resources: registered.outputs.get("detachedResources")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            duration: registered.outputs.get("duration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            error: registered.outputs.get("error")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            failed_resources: registered.outputs.get("failedResources")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            outputs: registered.outputs.get("outputs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            parameters: registered.outputs.get("parameters")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            parameters_link: registered.outputs.get("parametersLink")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resources: registered.outputs.get("resources")
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
        })
    }
}
