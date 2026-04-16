use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a nat ruleGet.
#[derive(Default)]
pub struct GetNatRuleArgs {
    /// The name of the gateway.
    pub gateway_name: String,
    /// The name of the nat rule.
    pub nat_rule_name: String,
    /// The resource group name of the VpnGateway.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNatRuleResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// List of egress VpnSiteLinkConnections.
    pub egress_vpn_site_link_connections: Vec<network::v20210501::SubResourceResponse>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The private IP address external mapping for NAT.
    pub external_mappings: Option<Vec<network::v20210501::VpnNatRuleMappingResponse>>,
    /// Resource ID.
    pub id: Option<String>,
    /// List of ingress VpnSiteLinkConnections.
    pub ingress_vpn_site_link_connections: Vec<network::v20210501::SubResourceResponse>,
    /// The private IP address internal mapping for NAT.
    pub internal_mappings: Option<Vec<network::v20210501::VpnNatRuleMappingResponse>>,
    /// The IP Configuration ID this NAT rule applies to.
    pub ip_configuration_id: Option<String>,
    /// The Source NAT direction of a VPN NAT.
    pub mode: Option<String>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The provisioning state of the NAT Rule resource.
    pub provisioning_state: String,
    /// Resource type.
    pub type_: String,
}

/// Retrieves the details of a nat ruleGet.
pub async fn get_nat_rule(
    ctx: &Context,
    args: GetNatRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNatRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("gatewayName".to_string(), json!(args.gateway_name));
    invoke_args.insert("natRuleName".to_string(), json!(args.nat_rule_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20210501:getNatRule", invoke_args, &opts).await?;

    Ok(GetNatRuleResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        egress_vpn_site_link_connections: serde_json::from_value(result.fields.get("egressVpnSiteLinkConnections").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        external_mappings: result.fields.get("externalMappings").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ingress_vpn_site_link_connections: serde_json::from_value(result.fields.get("ingressVpnSiteLinkConnections").cloned().unwrap_or_default())?
            ,
        internal_mappings: result.fields.get("internalMappings").cloned().map(serde_json::from_value).transpose()?,
        ip_configuration_id: result.fields.get("ipConfigurationId").cloned().map(serde_json::from_value).transpose()?,
        mode: result.fields.get("mode").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
