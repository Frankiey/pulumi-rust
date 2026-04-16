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
    /// Enable/Disable Copy/Paste feature of the Bastion Host resource.
    pub disable_copy_paste: Option<bool>,
    /// FQDN for the endpoint on which bastion host is accessible.
    pub dns_name: Option<String>,
    /// Enable/Disable File Copy feature of the Bastion Host resource.
    pub enable_file_copy: Option<bool>,
    /// Enable/Disable IP Connect feature of the Bastion Host resource.
    pub enable_ip_connect: Option<bool>,
    /// Enable/Disable Kerberos feature of the Bastion Host resource.
    pub enable_kerberos: Option<bool>,
    /// Enable/Disable Shareable Link of the Bastion Host resource.
    pub enable_shareable_link: Option<bool>,
    /// Enable/Disable Tunneling feature of the Bastion Host resource.
    pub enable_tunneling: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// IP configuration of the Bastion Host resource.
    pub ip_configurations: Option<Vec<network::v20230401::BastionHostIPConfigurationResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the bastion host resource.
    pub provisioning_state: String,
    /// The scale units for the Bastion Host resource.
    pub scale_units: Option<i64>,
    /// The sku of this Bastion Host.
    pub sku: Option<network::v20230401::SkuResponse>,
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
    let result = ctx.invoke("azure-native:network/v20230401:getBastionHost", invoke_args, &opts).await?;

    Ok(GetBastionHostResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        disable_copy_paste: result.fields.get("disableCopyPaste").cloned().map(serde_json::from_value).transpose()?,
        dns_name: result.fields.get("dnsName").cloned().map(serde_json::from_value).transpose()?,
        enable_file_copy: result.fields.get("enableFileCopy").cloned().map(serde_json::from_value).transpose()?,
        enable_ip_connect: result.fields.get("enableIpConnect").cloned().map(serde_json::from_value).transpose()?,
        enable_kerberos: result.fields.get("enableKerberos").cloned().map(serde_json::from_value).transpose()?,
        enable_shareable_link: result.fields.get("enableShareableLink").cloned().map(serde_json::from_value).transpose()?,
        enable_tunneling: result.fields.get("enableTunneling").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        scale_units: result.fields.get("scaleUnits").cloned().map(serde_json::from_value).transpose()?,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
