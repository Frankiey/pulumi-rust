use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the details of a nat rule.
#[derive(Default)]
pub struct GetVirtualNetworkGatewayNatRuleArgs {
    /// The name of the nat rule.
    pub nat_rule_name: String,
    /// The resource group name of the Virtual Network Gateway.
    pub resource_group_name: String,
    /// The name of the gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayNatRuleResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The private IP address external mapping for NAT.
    pub external_mappings: Option<Vec<network::v20250501::VpnNatRuleMappingResponse>>,
    /// Resource ID.
    pub id: Option<String>,
    /// The private IP address internal mapping for NAT.
    pub internal_mappings: Option<Vec<network::v20250501::VpnNatRuleMappingResponse>>,
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

/// Retrieves the details of a nat rule.
pub async fn get_virtual_network_gateway_nat_rule(
    ctx: &Context,
    args: GetVirtualNetworkGatewayNatRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayNatRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("natRuleName".to_string(), json!(args.nat_rule_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250501:getVirtualNetworkGatewayNatRule", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayNatRuleResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        external_mappings: result.fields.get("externalMappings").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
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
