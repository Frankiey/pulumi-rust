use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the requested ExpressRoutePort resource.
#[derive(Default)]
pub struct GetExpressRoutePortArgs {
    /// The name of ExpressRoutePort.
    pub express_route_port_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetExpressRoutePortResult {
    /// Date of the physical port allocation to be used in Letter of Authorization.
    pub allocation_date: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Bandwidth of procured ports in Gbps.
    pub bandwidth_in_gbps: Option<i64>,
    /// Reference the ExpressRoute circuit(s) that are provisioned on this ExpressRoutePort resource.
    pub circuits: Vec<network::v20191201::SubResourceResponse>,
    /// Encapsulation method on physical ports.
    pub encapsulation: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Ether type of the physical port.
    pub ether_type: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The identity of ExpressRoutePort, if configured.
    pub identity: Option<network::v20191201::ManagedServiceIdentityResponse>,
    /// The set of physical links of the ExpressRoutePort resource.
    pub links: Option<Vec<network::v20191201::ExpressRouteLinkResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Maximum transmission unit of the physical port pair(s).
    pub mtu: String,
    /// Resource name.
    pub name: String,
    /// The name of the peering location that the ExpressRoutePort is mapped to physically.
    pub peering_location: Option<String>,
    /// Aggregate Gbps of associated circuit bandwidths.
    pub provisioned_bandwidth_in_gbps: f64,
    /// The provisioning state of the express route port resource.
    pub provisioning_state: String,
    /// The resource GUID property of the express route port resource.
    pub resource_guid: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Retrieves the requested ExpressRoutePort resource.
pub async fn get_express_route_port(
    ctx: &Context,
    args: GetExpressRoutePortArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetExpressRoutePortResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("expressRoutePortName".to_string(), json!(args.express_route_port_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20191201:getExpressRoutePort", invoke_args, &opts).await?;

    Ok(GetExpressRoutePortResult {
        allocation_date: serde_json::from_value(result.fields.get("allocationDate").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        bandwidth_in_gbps: result.fields.get("bandwidthInGbps").cloned().map(serde_json::from_value).transpose()?,
        circuits: serde_json::from_value(result.fields.get("circuits").cloned().unwrap_or_default())?
            ,
        encapsulation: result.fields.get("encapsulation").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        ether_type: serde_json::from_value(result.fields.get("etherType").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        links: result.fields.get("links").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        mtu: serde_json::from_value(result.fields.get("mtu").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        peering_location: result.fields.get("peeringLocation").cloned().map(serde_json::from_value).transpose()?,
        provisioned_bandwidth_in_gbps: serde_json::from_value(result.fields.get("provisionedBandwidthInGbps").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
