use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the private dns zone group resource by specified private dns zone group name.
#[derive(Default)]
pub struct GetPrivateDnsZoneGroupArgs {
    /// The name of the private dns zone group.
    pub private_dns_zone_group_name: String,
    /// The name of the private endpoint.
    pub private_endpoint_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetPrivateDnsZoneGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// A collection of private dns zone configurations of the private dns zone group.
    pub private_dns_zone_configs: Option<Vec<network::v20230201::PrivateDnsZoneConfigResponse>>,
    /// The provisioning state of the private dns zone group resource.
    pub provisioning_state: String,
}

/// Gets the private dns zone group resource by specified private dns zone group name.
pub async fn get_private_dns_zone_group(
    ctx: &Context,
    args: GetPrivateDnsZoneGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetPrivateDnsZoneGroupResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("privateDnsZoneGroupName".to_string(), json!(args.private_dns_zone_group_name));
    invoke_args.insert("privateEndpointName".to_string(), json!(args.private_endpoint_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230201:getPrivateDnsZoneGroup", invoke_args, &opts).await?;

    Ok(GetPrivateDnsZoneGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        private_dns_zone_configs: result.fields.get("privateDnsZoneConfigs").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
    })
}
