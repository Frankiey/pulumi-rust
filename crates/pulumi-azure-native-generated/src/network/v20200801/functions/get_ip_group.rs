use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified ipGroups.
#[derive(Default)]
pub struct GetIpGroupArgs {
    /// Expands resourceIds (of Firewalls/Network Security Groups etc.) back referenced by the IpGroups resource.
    pub expand: Option<String>,
    /// The name of the ipGroups.
    pub ip_groups_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetIpGroupResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// List of references to Firewall Policies resources that this IpGroups is associated with.
    pub firewall_policies: Vec<network::v20200801::SubResourceResponse>,
    /// List of references to Firewall resources that this IpGroups is associated with.
    pub firewalls: Vec<network::v20200801::SubResourceResponse>,
    /// Resource ID.
    pub id: Option<String>,
    /// IpAddresses/IpAddressPrefixes in the IpGroups resource.
    pub ip_addresses: Option<Vec<String>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the IpGroups resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified ipGroups.
pub async fn get_ip_group(
    ctx: &Context,
    args: GetIpGroupArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetIpGroupResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("ipGroupsName".to_string(), json!(args.ip_groups_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200801:getIpGroup", invoke_args, &opts).await?;

    Ok(GetIpGroupResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        firewall_policies: serde_json::from_value(result.fields.get("firewallPolicies").cloned().unwrap_or_default())?
            ,
        firewalls: serde_json::from_value(result.fields.get("firewalls").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_addresses: result.fields.get("ipAddresses").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
