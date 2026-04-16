use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Security Partner Provider.
#[derive(Default)]
pub struct GetSecurityPartnerProviderArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the Security Partner Provider.
    pub security_partner_provider_name: String,
}

/// Result of the function invocation.
pub struct GetSecurityPartnerProviderResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The connection status with the Security Partner Provider.
    pub connection_status: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the Security Partner Provider resource.
    pub provisioning_state: String,
    /// The security provider name.
    pub security_provider_name: Option<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The virtualHub to which the Security Partner Provider belongs.
    pub virtual_hub: Option<network::v20230601::SubResourceResponse>,
}

/// Gets the specified Security Partner Provider.
pub async fn get_security_partner_provider(
    ctx: &Context,
    args: GetSecurityPartnerProviderArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetSecurityPartnerProviderResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("securityPartnerProviderName".to_string(), json!(args.security_partner_provider_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230601:getSecurityPartnerProvider", invoke_args, &opts).await?;

    Ok(GetSecurityPartnerProviderResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        connection_status: serde_json::from_value(result.fields.get("connectionStatus").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        security_provider_name: result.fields.get("securityProviderName").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hub: result.fields.get("virtualHub").cloned().map(serde_json::from_value).transpose()?,
    })
}
