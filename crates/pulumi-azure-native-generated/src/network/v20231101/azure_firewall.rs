use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Azure Firewall resource.
pub struct AzureFirewallArgs {
    /// The additional properties used to further config this azure firewall.
    pub additional_properties: Option<HashMap<String, Input<String>>>,
    /// Collection of application rule collections used by Azure Firewall.
    pub application_rule_collections: Option<Vec<Input<network::v20231101::AzureFirewallApplicationRuleCollectionArgs>>>,
    /// The name of the Azure Firewall.
    pub azure_firewall_name: Option<Input<String>>,
    /// The firewallPolicy associated with this azure firewall.
    pub firewall_policy: Option<Input<network::v20231101::SubResourceArgs>>,
    /// IP addresses associated with AzureFirewall.
    pub hub_ip_addresses: Option<Input<network::v20231101::HubIPAddressesArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// IP configuration of the Azure Firewall resource.
    pub ip_configurations: Option<Vec<Input<network::v20231101::AzureFirewallIPConfigurationArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// IP configuration of the Azure Firewall used for management traffic.
    pub management_ip_configuration: Option<Input<network::v20231101::AzureFirewallIPConfigurationArgs>>,
    /// Collection of NAT rule collections used by Azure Firewall.
    pub nat_rule_collections: Option<Vec<Input<network::v20231101::AzureFirewallNatRuleCollectionArgs>>>,
    /// Collection of network rule collections used by Azure Firewall.
    pub network_rule_collections: Option<Vec<Input<network::v20231101::AzureFirewallNetworkRuleCollectionArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The Azure Firewall Resource SKU.
    pub sku: Option<Input<network::v20231101::AzureFirewallSkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Option<Input<serde_json::Value>>,
    /// The virtualHub to which the firewall belongs.
    pub virtual_hub: Option<Input<network::v20231101::SubResourceArgs>>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Option<Vec<Input<String>>>,
}

/// Azure Firewall resource.
pub struct AzureFirewall {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The additional properties used to further config this azure firewall.
    pub additional_properties: Output<serde_json::Value>,
    /// Collection of application rule collections used by Azure Firewall.
    pub application_rule_collections: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The firewallPolicy associated with this azure firewall.
    pub firewall_policy: Output<serde_json::Value>,
    /// IP addresses associated with AzureFirewall.
    pub hub_ip_addresses: Output<serde_json::Value>,
    /// IP configuration of the Azure Firewall resource.
    pub ip_configurations: Output<serde_json::Value>,
    /// IpGroups associated with AzureFirewall.
    pub ip_groups: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// IP configuration of the Azure Firewall used for management traffic.
    pub management_ip_configuration: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// Collection of NAT rule collections used by Azure Firewall.
    pub nat_rule_collections: Output<serde_json::Value>,
    /// Collection of network rule collections used by Azure Firewall.
    pub network_rule_collections: Output<serde_json::Value>,
    /// The provisioning state of the Azure firewall resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The Azure Firewall Resource SKU.
    pub sku: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The virtualHub to which the firewall belongs.
    pub virtual_hub: Output<serde_json::Value>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Output<serde_json::Value>,
}

impl AzureFirewall {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20231101:AzureFirewall";

    /// Create a new AzureFirewall resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: AzureFirewallArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.additional_properties {
            pulumi_sdk::resolve_input_map("additionalProperties", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.application_rule_collections {
            pulumi_sdk::resolve_input_vec("applicationRuleCollections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.azure_firewall_name {
            pulumi_sdk::resolve_input("azureFirewallName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.firewall_policy {
            pulumi_sdk::resolve_input("firewallPolicy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.hub_ip_addresses {
            pulumi_sdk::resolve_input("hubIPAddresses", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_configurations {
            pulumi_sdk::resolve_input_vec("ipConfigurations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.management_ip_configuration {
            pulumi_sdk::resolve_input("managementIpConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nat_rule_collections {
            pulumi_sdk::resolve_input_vec("natRuleCollections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_rule_collections {
            pulumi_sdk::resolve_input_vec("networkRuleCollections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.threat_intel_mode {
            pulumi_sdk::resolve_input("threatIntelMode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_hub {
            pulumi_sdk::resolve_input("virtualHub", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.zones {
            pulumi_sdk::resolve_input_vec("zones", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }

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
            additional_properties: registered.outputs.get("additionalProperties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            application_rule_collections: registered.outputs.get("applicationRuleCollections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            firewall_policy: registered.outputs.get("firewallPolicy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            hub_ip_addresses: registered.outputs.get("hubIPAddresses")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_configurations: registered.outputs.get("ipConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_groups: registered.outputs.get("ipGroups")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            management_ip_configuration: registered.outputs.get("managementIpConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            nat_rule_collections: registered.outputs.get("natRuleCollections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_rule_collections: registered.outputs.get("networkRuleCollections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            threat_intel_mode: registered.outputs.get("threatIntelMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_hub: registered.outputs.get("virtualHub")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
