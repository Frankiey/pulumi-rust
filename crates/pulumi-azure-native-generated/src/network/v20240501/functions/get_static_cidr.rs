use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Instance of StaticCidr resource.
#[derive(Default)]
pub struct GetStaticCidrArgs {
    /// The name of the network manager.
    pub network_manager_name: String,
    /// Pool resource name.
    pub pool_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// StaticCidr resource name to retrieve.
    pub static_cidr_name: String,
}

/// Result of the function invocation.
pub struct GetStaticCidrResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// Properties of static CIDR resource.
    pub properties: network::v20240501::StaticCidrPropertiesResponse,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::v20240501::SystemDataResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Instance of StaticCidr resource.
pub async fn get_static_cidr(
    ctx: &Context,
    args: GetStaticCidrArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetStaticCidrResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkManagerName".to_string(), json!(args.network_manager_name));
    invoke_args.insert("poolName".to_string(), json!(args.pool_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("staticCidrName".to_string(), json!(args.static_cidr_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20240501:getStaticCidr", invoke_args, &opts).await?;

    Ok(GetStaticCidrResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
