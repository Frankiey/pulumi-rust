use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Lists IKE Security Associations for Vpn Site Link Connection in the specified resource group.
#[derive(Default)]
pub struct GetVpnLinkConnectionIkeSasArgs {
    /// The name of the vpn connection.
    pub connection_name: String,
    /// The name of the gateway.
    pub gateway_name: String,
    /// The name of the vpn link connection.
    pub link_connection_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetVpnLinkConnectionIkeSasResult {
    pub value: Option<String>,
}

/// Lists IKE Security Associations for Vpn Site Link Connection in the specified resource group.
pub async fn get_vpn_link_connection_ike_sas(
    ctx: &Context,
    args: GetVpnLinkConnectionIkeSasArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVpnLinkConnectionIkeSasResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("gatewayName".to_string(), json!(args.gateway_name));
    invoke_args.insert("linkConnectionName".to_string(), json!(args.link_connection_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230601:getVpnLinkConnectionIkeSas", invoke_args, &opts).await?;

    Ok(GetVpnLinkConnectionIkeSasResult {
        value: result.fields.get("value").cloned().map(serde_json::from_value).transpose()?,
    })
}
