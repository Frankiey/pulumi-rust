use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified service gateway.
#[derive(Default)]
pub struct GetServiceGatewayArgs {
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
    /// The name of the service gateway.
    pub service_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetServiceGatewayResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The geo-location where the resource lives
    pub location: String,
    /// The name of the resource
    pub name: String,
    /// The provisioning state of the service gateway resource.
    pub provisioning_state: String,
    /// The resource GUID property of the service gateway resource.
    pub resource_guid: String,
    /// Route Target address of Service gateway
    pub route_target_address: Option<network::v20250501::RouteTargetAddressPropertiesFormatResponse>,
    /// Route Target address V6 of Service gateway
    pub route_target_address_v6: Option<network::v20250501::RouteTargetAddressPropertiesFormatResponse>,
    /// The service gateway SKU.
    pub sku: Option<network::v20250501::ServiceGatewaySkuResponse>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::v20250501::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
    /// Reference to an existing virtual network.
    pub virtual_network: Option<network::v20250501::VirtualNetworkResponse>,
    /// A list of availability zones denoting the zone in which service gateway should be deployed. 
    pub zones: Option<Vec<String>>,
}

/// Gets the specified service gateway.
pub async fn get_service_gateway(
    ctx: &Context,
    args: GetServiceGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetServiceGatewayResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("serviceGatewayName".to_string(), json!(args.service_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250501:getServiceGateway", invoke_args, &opts).await?;

    Ok(GetServiceGatewayResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        route_target_address: result.fields.get("routeTargetAddress").cloned().map(serde_json::from_value).transpose()?,
        route_target_address_v6: result.fields.get("routeTargetAddressV6").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_network: result.fields.get("virtualNetwork").cloned().map(serde_json::from_value).transpose()?,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
