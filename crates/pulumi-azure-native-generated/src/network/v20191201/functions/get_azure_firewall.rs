use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified Azure Firewall.
#[derive(Default)]
pub struct GetAzureFirewallArgs {
    /// The name of the Azure Firewall.
    pub azure_firewall_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetAzureFirewallResult {
    /// The additional properties used to further config this azure firewall.
    pub additional_properties: Option<HashMap<String, String>>,
    /// Collection of application rule collections used by Azure Firewall.
    pub application_rule_collections: Option<Vec<network::v20191201::AzureFirewallApplicationRuleCollectionResponse>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// The firewallPolicy associated with this azure firewall.
    pub firewall_policy: Option<network::v20191201::SubResourceResponse>,
    /// IP addresses associated with AzureFirewall.
    pub hub_ip_addresses: network::v20191201::HubIPAddressesResponse,
    /// Resource ID.
    pub id: Option<String>,
    /// IP configuration of the Azure Firewall resource.
    pub ip_configurations: Option<Vec<network::v20191201::AzureFirewallIPConfigurationResponse>>,
    /// IpGroups associated with AzureFirewall.
    pub ip_groups: Vec<network::v20191201::AzureFirewallIpGroupsResponse>,
    /// Resource location.
    pub location: Option<String>,
    /// IP configuration of the Azure Firewall used for management traffic.
    pub management_ip_configuration: Option<network::v20191201::AzureFirewallIPConfigurationResponse>,
    /// Resource name.
    pub name: String,
    /// Collection of NAT rule collections used by Azure Firewall.
    pub nat_rule_collections: Option<Vec<network::v20191201::AzureFirewallNatRuleCollectionResponse>>,
    /// Collection of network rule collections used by Azure Firewall.
    pub network_rule_collections: Option<Vec<network::v20191201::AzureFirewallNetworkRuleCollectionResponse>>,
    /// The provisioning state of the Azure firewall resource.
    pub provisioning_state: String,
    /// The Azure Firewall Resource SKU.
    pub sku: Option<network::v20191201::AzureFirewallSkuResponse>,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Option<String>,
    /// Resource type.
    pub type_: String,
    /// The virtualHub to which the firewall belongs.
    pub virtual_hub: Option<network::v20191201::SubResourceResponse>,
    /// A list of availability zones denoting where the resource needs to come from.
    pub zones: Option<Vec<String>>,
}

/// Gets the specified Azure Firewall.
pub async fn get_azure_firewall(
    ctx: &Context,
    args: GetAzureFirewallArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetAzureFirewallResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("azureFirewallName".to_string(), json!(args.azure_firewall_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20191201:getAzureFirewall", invoke_args, &opts).await?;

    Ok(GetAzureFirewallResult {
        additional_properties: result.fields.get("additionalProperties").cloned().map(serde_json::from_value).transpose()?,
        application_rule_collections: result.fields.get("applicationRuleCollections").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        firewall_policy: result.fields.get("firewallPolicy").cloned().map(serde_json::from_value).transpose()?,
        hub_ip_addresses: serde_json::from_value(result.fields.get("hubIpAddresses").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        ip_groups: serde_json::from_value(result.fields.get("ipGroups").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        management_ip_configuration: result.fields.get("managementIpConfiguration").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        nat_rule_collections: result.fields.get("natRuleCollections").cloned().map(serde_json::from_value).transpose()?,
        network_rule_collections: result.fields.get("networkRuleCollections").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        sku: result.fields.get("sku").cloned().map(serde_json::from_value).transpose()?,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        threat_intel_mode: result.fields.get("threatIntelMode").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_hub: result.fields.get("virtualHub").cloned().map(serde_json::from_value).transpose()?,
        zones: result.fields.get("zones").cloned().map(serde_json::from_value).transpose()?,
    })
}
