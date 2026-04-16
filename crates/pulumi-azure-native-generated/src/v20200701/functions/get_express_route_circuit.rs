use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets information about the specified express route circuit.
#[derive(Default)]
pub struct GetExpressRouteCircuitArgs {
    /// The name of express route circuit.
    pub circuit_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRouteCircuitResult {
    /// Allow classic operations.
    pub allow_classic_operations: Option<bool>,
    /// The list of authorizations.
    pub authorizations: Option<Vec<network::v20200701::ExpressRouteCircuitAuthorizationResponse>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The bandwidth of the circuit when the circuit is provisioned on an ExpressRoutePort resource.
    pub bandwidth_in_gbps: Option<f64>,
    /// The CircuitProvisioningState state of the resource.
    pub circuit_provisioning_state: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The reference to the ExpressRoutePort resource when the circuit is provisioned on an ExpressRoutePort resource.
    pub express_route_port: Option<network::v20200701::SubResourceResponse>,
    /// The GatewayManager Etag.
    pub gateway_manager_etag: Option<String>,
    /// Flag denoting global reach status.
    pub global_reach_enabled: Option<bool>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The list of peerings.
    pub peerings: Option<Vec<network::v20200701::ExpressRouteCircuitPeeringResponse>>,
    /// The provisioning state of the express route circuit resource.
    pub provisioning_state: String,
    /// The ServiceKey.
    pub service_key: Option<String>,
    /// The ServiceProviderNotes.
    pub service_provider_notes: Option<String>,
    /// The ServiceProviderProperties.
    pub service_provider_properties: Option<network::v20200701::ExpressRouteCircuitServiceProviderPropertiesResponse>,
    /// The ServiceProviderProvisioningState state of the resource.
    pub service_provider_provisioning_state: Option<String>,
    /// The SKU.
    pub sku: Option<network::v20200701::ExpressRouteCircuitSkuResponse>,
    /// The identifier of the circuit traffic. Outer tag for QinQ encapsulation.
    pub stag: i64,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets information about the specified express route circuit.
pub async fn get_express_route_circuit(
    ctx: &Context,
    args: GetExpressRouteCircuitArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRouteCircuitResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("circuitName".to_string(), json!(args.circuit_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200701:getExpressRouteCircuit", invoke_args, &opts).await?;

    Ok(GetExpressRouteCircuitResult {
        allow_classic_operations: result.fields.get("allowClassicOperations").cloned().map(serde_json::from_value).transpose()?,
        authorizations: result.fields.get("authorizations").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bandwidth_in_gbps: result.fields.get("bandwidthInGbps").cloned().map(serde_json::from_value).transpose()?,
        circuit_provisioning_state: result.fields.get("circuitProvisioningState").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        express_route_port: result.fields.get("expressRoutePort").cloned().map(serde_json::from_value).transpose()?,
        gateway_manager_etag: result.fields.get("gatewayManagerEtag").cloned().map(serde_json::from_value).transpose()?,
        global_reach_enabled: result.fields.get("globalReachEnabled").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        peerings: result.fields.get("peerings").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        service_key: result.fields.get("serviceKey").cloned().map(serde_json::from_value).transpose()?,
        service_provider_notes: result.fields.get("serviceProviderNotes").cloned().map(serde_json::from_value).transpose()?,
        service_provider_properties: result.fields.get("serviceProviderProperties").cloned().map(serde_json::from_value).transpose()?,
        service_provider_provisioning_state: result.fields.get("serviceProviderProvisioningState").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        stag: serde_json::from_value(result.fields.get("stag").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
