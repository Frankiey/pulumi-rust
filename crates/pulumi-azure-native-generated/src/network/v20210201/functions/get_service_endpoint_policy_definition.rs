use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Get the specified service endpoint policy definitions from service endpoint policy.
#[derive(Default)]
pub struct GetServiceEndpointPolicyDefinitionArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the service endpoint policy definition name.
    pub service_endpoint_policy_definition_name: String,
    /// The name of the service endpoint policy name.
    pub service_endpoint_policy_name: String,
}

/// Result of the function invocation.
pub struct GetServiceEndpointPolicyDefinitionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A description for this rule. Restricted to 140 chars.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the service endpoint policy definition resource.
    pub provisioning_state: String,
    /// Service endpoint name.
    pub service: Option<String>,
    /// A list of service resources.
    pub service_resources: Option<Vec<String>>,
}

/// Get the specified service endpoint policy definitions from service endpoint policy.
pub async fn get_service_endpoint_policy_definition(
    ctx: &Context,
    args: GetServiceEndpointPolicyDefinitionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetServiceEndpointPolicyDefinitionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("serviceEndpointPolicyDefinitionName".to_string(), json!(args.service_endpoint_policy_definition_name));
    invoke_args.insert("serviceEndpointPolicyName".to_string(), json!(args.service_endpoint_policy_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20210201:getServiceEndpointPolicyDefinition", invoke_args, &opts).await?;

    Ok(GetServiceEndpointPolicyDefinitionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        service: result.fields.get("service").cloned().map(serde_json::from_value).transpose()?,
        service_resources: result.fields.get("serviceResources").cloned().map(serde_json::from_value).transpose()?,
    })
}
