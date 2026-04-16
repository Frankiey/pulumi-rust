use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Bastion Host.
#[derive(Default)]
pub struct GetBastionHostArgs {
    /// The name of the Bastion Host.
    pub bastion_host_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetBastionHostResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// FQDN for the endpoint on which bastion host is accessible.
    pub dns_name: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// IP configuration of the Bastion Host resource.
    pub ip_configurations: Option<Vec<network::v20210201::BastionHostIPConfigurationResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the bastion host resource.
    pub provisioning_state: String,
    /// The sku of this Bastion Host.
    pub sku: Option<network::v20210201::SkuResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified Bastion Host.
pub async fn get_bastion_host(
    ctx: &Context,
    args: GetBastionHostArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetBastionHostResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("bastionHostName".to_string(), json!(args.bastion_host_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20210201:getBastionHost", invoke_args, &opts).await?;

    Ok(GetBastionHostResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        dns_name: result.fields.get("dnsName").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
