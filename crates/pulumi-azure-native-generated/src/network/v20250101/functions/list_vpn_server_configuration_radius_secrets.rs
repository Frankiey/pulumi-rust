use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// List all Radius servers with respective radius secrets from VpnServerConfiguration.
#[derive(Default)]
pub struct ListVpnServerConfigurationRadiusSecretsArgs {
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
    /// The name of the VpnServerConfiguration.
    pub vpn_server_configuration_name: String,
}

/// Result of the function invocation.
pub struct ListVpnServerConfigurationRadiusSecretsResult {
    /// URL to get the next set of operation list results if there are any.
    pub next_link: Option<String>,
    /// List of Radius servers with respective radius secrets.
    pub value: Option<Vec<network::v20250101::RadiusAuthServerResponse>>,
}

/// List all Radius servers with respective radius secrets from VpnServerConfiguration.
pub async fn list_vpn_server_configuration_radius_secrets(
    ctx: &Context,
    args: ListVpnServerConfigurationRadiusSecretsArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListVpnServerConfigurationRadiusSecretsResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("vpnServerConfigurationName".to_string(), json!(args.vpn_server_configuration_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250101:listVpnServerConfigurationRadiusSecrets", invoke_args, &opts).await?;

    Ok(ListVpnServerConfigurationRadiusSecretsResult {
        next_link: result.fields.get("nextLink").cloned().map(serde_json::from_value).transpose()?,
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
