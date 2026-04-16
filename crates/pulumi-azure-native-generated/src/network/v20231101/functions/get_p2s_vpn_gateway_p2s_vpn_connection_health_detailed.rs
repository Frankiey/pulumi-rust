use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the sas url to get the connection health detail of P2S clients of the virtual wan P2SVpnGateway in the specified resource group.
#[derive(Default)]
pub struct GetP2sVpnGatewayP2sVpnConnectionHealthDetailedArgs {
    /// The name of the P2SVpnGateway.
    pub gateway_name: String,
    /// The sas-url to download the P2S Vpn connection health detail.
    pub output_blob_sas_url: Option<String>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The list of p2s vpn user names whose p2s vpn connection detailed health to retrieve for.
    pub vpn_user_names_filter: Option<Vec<String>>,
}

/// Result of the function invocation.
pub struct GetP2sVpnGatewayP2sVpnConnectionHealthDetailedResult {
    /// Returned sas url of the blob to which the p2s vpn connection detailed health will be written.
    pub sas_url: Option<String>,
}

/// Gets the sas url to get the connection health detail of P2S clients of the virtual wan P2SVpnGateway in the specified resource group.
pub async fn get_p2s_vpn_gateway_p2s_vpn_connection_health_detailed(
    ctx: &Context,
    args: GetP2sVpnGatewayP2sVpnConnectionHealthDetailedArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetP2sVpnGatewayP2sVpnConnectionHealthDetailedResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("gatewayName".to_string(), json!(args.gateway_name));
    if let Some(v) = args.output_blob_sas_url {
        invoke_args.insert("outputBlobSasUrl".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.vpn_user_names_filter {
        invoke_args.insert("vpnUserNamesFilter".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20231101:getP2sVpnGatewayP2sVpnConnectionHealthDetailed", invoke_args, &opts).await?;

    Ok(GetP2sVpnGatewayP2sVpnConnectionHealthDetailedResult {
        sas_url: result.fields.get("sasUrl").cloned().map(serde_json::from_value).transpose()?,
    })
}
