use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified private endpoint connection associated with the storage account.
#[derive(Default)]
pub struct GetPrivateEndpointConnectionArgs {
    /// The name of the storage account within the specified resource group. Storage account names must be between 3 and 24 characters in length and use numbers and lower-case letters only.
    pub account_name: String,
    /// The name of the private endpoint connection associated with the Azure resource
    pub private_endpoint_connection_name: String,
    /// The name of the resource group within the user's subscription. The name is case insensitive.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetPrivateEndpointConnectionResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. Ex - /subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// The resource of private end point.
    pub private_endpoint: Option<storage::v20230101::PrivateEndpointResponse>,
    /// A collection of information about the state of the connection between service consumer and provider.
    pub private_link_service_connection_state: storage::v20230101::PrivateLinkServiceConnectionStateResponse,
    /// The provisioning state of the private endpoint connection resource.
    pub provisioning_state: String,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the specified private endpoint connection associated with the storage account.
pub async fn get_private_endpoint_connection(
    ctx: &Context,
    args: GetPrivateEndpointConnectionArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPrivateEndpointConnectionResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accountName".to_string(), json!(args.account_name));
    invoke_args.insert("privateEndpointConnectionName".to_string(), json!(args.private_endpoint_connection_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:storage/v20230101:getPrivateEndpointConnection", invoke_args, &opts).await?;

    Ok(GetPrivateEndpointConnectionResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        private_endpoint: result.fields.get("privateEndpoint").cloned().map(serde_json::from_value).transpose()?,
        private_link_service_connection_state: serde_json::from_value(result.fields.get("privateLinkServiceConnectionState").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
