use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Virtual Router.
#[derive(Default)]
pub struct GetVirtualRouterArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the Virtual Router.
    pub virtual_router_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualRouterResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The Gateway on which VirtualRouter is hosted.
    pub hosted_gateway: Option<network::v20200401::SubResourceResponse>,
    /// The Subnet on which VirtualRouter is hosted.
    pub hosted_subnet: Option<network::v20200401::SubResourceResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// List of references to VirtualRouterPeerings.
    pub peerings: Vec<network::v20200401::SubResourceResponse>,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// VirtualRouter ASN.
    pub virtual_router_asn: Option<f64>,
    /// VirtualRouter IPs.
    pub virtual_router_ips: Option<Vec<String>>,
}

/// Gets the specified Virtual Router.
pub async fn get_virtual_router(
    ctx: &Context,
    args: GetVirtualRouterArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualRouterResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualRouterName".to_string(), json!(args.virtual_router_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200401:getVirtualRouter", invoke_args, &opts).await?;

    Ok(GetVirtualRouterResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        hosted_gateway: result.fields.get("hostedGateway").cloned().map(serde_json::from_value).transpose()?,
        hosted_subnet: result.fields.get("hostedSubnet").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        peerings: serde_json::from_value(result.fields.get("peerings").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_router_asn: result.fields.get("virtualRouterAsn").cloned().map(serde_json::from_value).transpose()?,
        virtual_router_ips: result.fields.get("virtualRouterIps").cloned().map(serde_json::from_value).transpose()?,
    })
}
