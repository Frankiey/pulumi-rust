use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified service Endpoint Policies in a specified resource group.
#[derive(Default)]
pub struct GetServiceEndpointPolicyArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the service endpoint policy.
    pub service_endpoint_policy_name: String,
}

/// Result of the function invocation.
pub struct GetServiceEndpointPolicyResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A collection of contextual service endpoint policy.
    pub contextual_service_endpoint_policies: Option<Vec<String>>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Kind of service endpoint policy. This is metadata used for the Azure portal experience.
    pub kind: String,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the service endpoint policy resource.
    pub provisioning_state: String,
    /// The resource GUID property of the service endpoint policy resource.
    pub resource_guid: String,
    /// The alias indicating if the policy belongs to a service
    pub service_alias: Option<String>,
    /// A collection of service endpoint policy definitions of the service endpoint policy.
    pub service_endpoint_policy_definitions: Option<Vec<network::v20210301::ServiceEndpointPolicyDefinitionResponse>>,
    /// A collection of references to subnets.
    pub subnets: Vec<network::v20210301::SubnetResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified service Endpoint Policies in a specified resource group.
pub async fn get_service_endpoint_policy(
    ctx: &Context,
    args: GetServiceEndpointPolicyArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetServiceEndpointPolicyResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("serviceEndpointPolicyName".to_string(), json!(args.service_endpoint_policy_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20210301:getServiceEndpointPolicy", invoke_args, &opts).await?;

    Ok(GetServiceEndpointPolicyResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        contextual_service_endpoint_policies: result.fields.get("contextualServiceEndpointPolicies").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        kind: serde_json::from_value(result.fields.get("kind").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        service_alias: result.fields.get("serviceAlias").cloned().map(serde_json::from_value).transpose()?,
        service_endpoint_policy_definitions: result.fields.get("serviceEndpointPolicyDefinitions").cloned().map(serde_json::from_value).transpose()?,
        subnets: serde_json::from_value(result.fields.get("subnets").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
