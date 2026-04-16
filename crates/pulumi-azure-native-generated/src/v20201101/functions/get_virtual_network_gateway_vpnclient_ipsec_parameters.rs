use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// The Get VpnclientIpsecParameters operation retrieves information about the vpnclient ipsec policy for P2S client of virtual network gateway in the specified resource group through Network resource provider.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayVpnclientIpsecParametersArgs {
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The virtual network gateway name.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayVpnclientIpsecParametersResult {
    /// The DH Group used in IKE Phase 1 for initial SA.
    pub dh_group: String,
    /// The IKE encryption algorithm (IKE phase 2).
    pub ike_encryption: String,
    /// The IKE integrity algorithm (IKE phase 2).
    pub ike_integrity: String,
    /// The IPSec encryption algorithm (IKE phase 1).
    pub ipsec_encryption: String,
    /// The IPSec integrity algorithm (IKE phase 1).
    pub ipsec_integrity: String,
    /// The Pfs Group used in IKE Phase 2 for new child SA.
    pub pfs_group: String,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) payload size in KB for P2S client..
    pub sa_data_size_kilobytes: i64,
    /// The IPSec Security Association (also called Quick Mode or Phase 2 SA) lifetime in seconds for P2S client.
    pub sa_life_time_seconds: i64,
}

/// The Get VpnclientIpsecParameters operation retrieves information about the vpnclient ipsec policy for P2S client of virtual network gateway in the specified resource group through Network resource provider.
pub async fn get_virtual_network_gateway_vpnclient_ipsec_parameters(
    ctx: &Context,
    args: GetVirtualNetworkGatewayVpnclientIpsecParametersArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayVpnclientIpsecParametersResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20201101:getVirtualNetworkGatewayVpnclientIpsecParameters", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayVpnclientIpsecParametersResult {
        dh_group: serde_json::from_value(result.fields.get("dhGroup").cloned().unwrap_or_default())?
            ,
        ike_encryption: serde_json::from_value(result.fields.get("ikeEncryption").cloned().unwrap_or_default())?
            ,
        ike_integrity: serde_json::from_value(result.fields.get("ikeIntegrity").cloned().unwrap_or_default())?
            ,
        ipsec_encryption: serde_json::from_value(result.fields.get("ipsecEncryption").cloned().unwrap_or_default())?
            ,
        ipsec_integrity: serde_json::from_value(result.fields.get("ipsecIntegrity").cloned().unwrap_or_default())?
            ,
        pfs_group: serde_json::from_value(result.fields.get("pfsGroup").cloned().unwrap_or_default())?
            ,
        sa_data_size_kilobytes: serde_json::from_value(result.fields.get("saDataSizeKilobytes").cloned().unwrap_or_default())?
            ,
        sa_life_time_seconds: serde_json::from_value(result.fields.get("saLifeTimeSeconds").cloned().unwrap_or_default())?
            ,
    })
}
