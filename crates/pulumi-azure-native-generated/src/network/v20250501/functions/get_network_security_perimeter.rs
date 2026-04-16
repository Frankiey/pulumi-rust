use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified network security perimeter by the name.
#[derive(Default)]
pub struct GetNetworkSecurityPerimeterArgs {
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkSecurityPerimeterResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The geo-location where the resource lives
    pub location: String,
    /// The name of the resource
    pub name: String,
    /// perimeter guid of the network security perimeter.
    pub perimeter_guid: String,
    /// The provisioning state of the scope assignment resource.
    pub provisioning_state: String,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::v20250501::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the specified network security perimeter by the name.
pub async fn get_network_security_perimeter(
    ctx: &Context,
    args: GetNetworkSecurityPerimeterArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkSecurityPerimeterResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkSecurityPerimeterName".to_string(), json!(args.network_security_perimeter_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250501:getNetworkSecurityPerimeter", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityPerimeterResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        perimeter_guid: serde_json::from_value(result.fields.get("perimeterGuid").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
