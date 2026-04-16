use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Virtual Appliance Site.
#[derive(Default)]
pub struct GetVirtualApplianceSiteArgs {
    /// The name of the Network Virtual Appliance.
    pub network_virtual_appliance_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the site.
    pub site_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualApplianceSiteResult {
    /// Address Prefix.
    pub address_prefix: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Name of the virtual appliance site.
    pub name: Option<String>,
    /// Office 365 Policy.
    pub o365policy: Option<network::v20200501::Office365PolicyPropertiesResponse>,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Site type.
    pub type_: String,
}

/// Gets the specified Virtual Appliance Site.
pub async fn get_virtual_appliance_site(
    ctx: &Context,
    args: GetVirtualApplianceSiteArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualApplianceSiteResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("networkVirtualApplianceName".to_string(), json!(args.network_virtual_appliance_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("siteName".to_string(), json!(args.site_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20200501:getVirtualApplianceSite", invoke_args, &opts).await?;

    Ok(GetVirtualApplianceSiteResult {
        address_prefix: result.fields.get("addressPrefix").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        o365policy: result.fields.get("o365Policy").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
