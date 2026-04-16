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
    /// Collection of application rule collections used by Azure Firewall.
    pub application_rule_collections: Option<Vec<network::v20190201::AzureFirewallApplicationRuleCollectionResponse>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: Option<String>,
    /// IP configuration of the Azure Firewall resource.
    pub ip_configurations: Option<Vec<network::v20190201::AzureFirewallIPConfigurationResponse>>,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// Collection of NAT rule collections used by Azure Firewall.
    pub nat_rule_collections: Option<Vec<network::v20190201::AzureFirewallNatRuleCollectionResponse>>,
    /// Collection of network rule collections used by Azure Firewall.
    pub network_rule_collections: Option<Vec<network::v20190201::AzureFirewallNetworkRuleCollectionResponse>>,
    /// The provisioning state of the resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// The operation mode for Threat Intelligence.
    pub threat_intel_mode: Option<String>,
    /// Resource type.
    pub type_: String,
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
    let result = ctx.invoke("azure-native:network/v20190201:getAzureFirewall", invoke_args, &opts).await?;

    Ok(GetAzureFirewallResult {
        application_rule_collections: result.fields.get("applicationRuleCollections").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        ip_configurations: result.fields.get("ipConfigurations").cloned().map(serde_json::from_value).transpose()?,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        nat_rule_collections: result.fields.get("natRuleCollections").cloned().map(serde_json::from_value).transpose()?,
        network_rule_collections: result.fields.get("networkRuleCollections").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        threat_intel_mode: result.fields.get("threatIntelMode").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
