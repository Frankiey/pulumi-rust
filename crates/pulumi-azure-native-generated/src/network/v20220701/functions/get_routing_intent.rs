use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a RoutingIntent.
#[derive(Default)]
pub struct GetRoutingIntentArgs {
    /// The resource group name of the RoutingIntent.
    pub resource_group_name: String,
    /// The name of the RoutingIntent.
    pub routing_intent_name: String,
    /// The name of the VirtualHub.
    pub virtual_hub_name: String,
}

/// Result of the function invocation.
pub struct GetRoutingIntentResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the RoutingIntent resource.
    pub provisioning_state: String,
    /// List of routing policies.
    pub routing_policies: Option<Vec<network::v20220701::RoutingPolicyResponse>>,
    /// Resource type.
    pub type_: String,
}

/// Retrieves the details of a RoutingIntent.
pub async fn get_routing_intent(
    ctx: &Context,
    args: GetRoutingIntentArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetRoutingIntentResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("routingIntentName".to_string(), json!(args.routing_intent_name));
    invoke_args.insert("virtualHubName".to_string(), json!(args.virtual_hub_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220701:getRoutingIntent", invoke_args, &opts).await?;

    Ok(GetRoutingIntentResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        routing_policies: result.fields.get("routingPolicies").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
