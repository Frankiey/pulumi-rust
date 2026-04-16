use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified custom IP prefix in a specified resource group.
#[derive(Default)]
pub struct GetCustomIPPrefixArgs {
    /// The name of the custom IP prefix.
    pub custom_ip_prefix_name: String,
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetCustomIPPrefixResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The prefix range in CIDR notation. Should include the start address and the prefix length.
    pub cidr: Option<String>,
    /// The commissioned state of the Custom IP Prefix.
    pub commissioned_state: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the custom IP prefix resource.
    pub provisioning_state: String,
    /// The list of all referenced PublicIpPrefixes.
    pub public_ip_prefixes: Vec<network::v20200701::SubResourceResponse>,
    /// The resource GUID property of the custom IP prefix resource.
    pub resource_guid: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Option<Vec<String>>,
}

/// Gets the specified custom IP prefix in a specified resource group.
pub async fn get_custom_ip_prefix(
    ctx: &Context,
    args: GetCustomIPPrefixArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetCustomIPPrefixResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("customIpPrefixName".to_string(), json!(args.custom_ip_prefix_name));
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200701:getCustomIPPrefix", invoke_args, &opts).await?;

    Ok(GetCustomIPPrefixResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        cidr: result.fields.get("cidr").cloned().map(serde_json::from_value).transpose()?,
        commissioned_state: result.fields.get("commissionedState").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        public_ip_prefixes: serde_json::from_value(result.fields.get("publicIpPrefixes").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
