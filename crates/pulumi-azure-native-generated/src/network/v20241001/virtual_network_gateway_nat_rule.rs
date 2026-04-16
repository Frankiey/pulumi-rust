use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VirtualNetworkGatewayNatRule Resource.
pub struct VirtualNetworkGatewayNatRuleArgs {
    /// The private IP address external mapping for NAT.
    pub external_mappings: Option<Vec<Input<network::v20241001::VpnNatRuleMappingArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The private IP address internal mapping for NAT.
    pub internal_mappings: Option<Vec<Input<network::v20241001::VpnNatRuleMappingArgs>>>,
    /// The IP Configuration ID this NAT rule applies to.
    pub ip_configuration_id: Option<Input<String>>,
    /// The Source NAT direction of a VPN NAT.
    pub mode: Option<Input<serde_json::Value>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the nat rule.
    pub nat_rule_name: Option<Input<String>>,
    /// The resource group name of the Virtual Network Gateway.
    pub resource_group_name: Input<String>,
    /// The type of NAT rule for VPN NAT.
    pub type_: Option<Input<serde_json::Value>>,
    /// The name of the gateway.
    pub virtual_network_gateway_name: Input<String>,
}

/// VirtualNetworkGatewayNatRule Resource.
pub struct VirtualNetworkGatewayNatRule {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The private IP address external mapping for NAT.
    pub external_mappings: Output<serde_json::Value>,
    /// The private IP address internal mapping for NAT.
    pub internal_mappings: Output<serde_json::Value>,
    /// The IP Configuration ID this NAT rule applies to.
    pub ip_configuration_id: Output<serde_json::Value>,
    /// The Source NAT direction of a VPN NAT.
    pub mode: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the NAT Rule resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl VirtualNetworkGatewayNatRule {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20241001:VirtualNetworkGatewayNatRule";

    /// Create a new VirtualNetworkGatewayNatRule resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualNetworkGatewayNatRuleArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.external_mappings {
            pulumi_sdk::resolve_input_vec("externalMappings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.internal_mappings {
            pulumi_sdk::resolve_input_vec("internalMappings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_configuration_id {
            pulumi_sdk::resolve_input("ipConfigurationId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.mode {
            pulumi_sdk::resolve_input("mode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nat_rule_name {
            pulumi_sdk::resolve_input("natRuleName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.type_ {
            pulumi_sdk::resolve_input("type", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualNetworkGatewayName", args.virtual_network_gateway_name, &mut inputs, &mut deps, &mut prop_deps).await;

        let registered = ctx.register_resource(
            Self::TYPE_TOKEN,
            name,
            inputs,
            prop_deps,
            &opts,
        ).await?;

        Ok(Self {
            urn: registered.urn.clone(),
            id: registered.outputs.get("id")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            external_mappings: registered.outputs.get("externalMappings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            internal_mappings: registered.outputs.get("internalMappings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configuration_id: registered.outputs.get("ipConfigurationId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            mode: registered.outputs.get("mode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
