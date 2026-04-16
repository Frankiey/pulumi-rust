use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified nat gateway in a specified resource group.
#[derive(Default)]
pub struct GetNatGatewayArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the nat gateway.
    pub nat_gateway_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNatGatewayResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// The idle timeout of the nat gateway.
    pub idle_timeout_in_minutes: Option<i64>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the NAT gateway resource.
    pub provisioning_state: String,
    /// An array of public ip addresses associated with the nat gateway resource.
    pub public_ip_addresses: Option<Vec<network::v20191101::SubResourceResponse>>,
    /// An array of public ip prefixes associated with the nat gateway resource.
    pub public_ip_prefixes: Option<Vec<network::v20191101::SubResourceResponse>>,
    /// The resource GUID property of the NAT gateway resource.
    pub resource_guid: String,
    /// The nat gateway SKU.
    pub sku: Option<network::v20191101::NatGatewaySkuResponse>,
    /// An array of references to the subnets using this nat gateway resource.
    pub subnets: Vec<network::v20191101::SubResourceResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// A list of availability zones denoting the zone in which Nat Gateway should be deployed.
    pub zones: Option<Vec<String>>,
}

/// Gets the specified nat gateway in a specified resource group.
pub async fn get_nat_gateway(
    ctx: &Context,
    args: GetNatGatewayArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNatGatewayResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("natGatewayName".to_string(), json!(args.nat_gateway_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20191101:getNatGateway", invoke_args, &opts).await?;

    Ok(GetNatGatewayResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        idle_timeout_in_minutes: result.fields.get("idleTimeoutInMinutes").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        public_ip_addresses: result.fields.get("publicIpAddresses").cloned().map(serde_json::from_value).transpose()?,
        public_ip_prefixes: result.fields.get("publicIpPrefixes").cloned().map(serde_json::from_value).transpose()?,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        subnets: serde_json::from_value(result.fields.get("subnets").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
