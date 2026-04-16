use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the value of the shared key of VpnLink connection specified.
#[derive(Default)]
pub struct ListVpnLinkConnectionDefaultSharedKeyArgs {
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
pub struct ListVpnLinkConnectionDefaultSharedKeyResult {
    /// Resource ID.
    pub id: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// Properties of the shared key.
    pub properties: network::v20250101::SharedKeyPropertiesResponse,
    /// Resource type.
    pub type_: String,
}

/// Gets the value of the shared key of VpnLink connection specified.
pub async fn list_vpn_link_connection_default_shared_key(
    ctx: &Context,
    args: ListVpnLinkConnectionDefaultSharedKeyArgs,
    opts: Option<InvokeOptions>,
) -> Result<ListVpnLinkConnectionDefaultSharedKeyResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("connectionName".to_string(), json!(args.connection_name));
    invoke_args.insert("gatewayName".to_string(), json!(args.gateway_name));
    invoke_args.insert("linkConnectionName".to_string(), json!(args.link_connection_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250101:listVpnLinkConnectionDefaultSharedKey", invoke_args, &opts).await?;

    Ok(ListVpnLinkConnectionDefaultSharedKeyResult {
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        properties: serde_json::from_value(result.fields.get("properties").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
