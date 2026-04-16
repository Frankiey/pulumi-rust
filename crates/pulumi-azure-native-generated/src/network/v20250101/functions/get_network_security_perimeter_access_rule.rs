use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified NSP access rule by name.
#[derive(Default)]
pub struct GetNetworkSecurityPerimeterAccessRuleArgs {
    /// The name of the NSP access rule.
    pub access_rule_name: String,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: String,
    /// The name of the NSP profile.
    pub profile_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkSecurityPerimeterAccessRuleResult {
    /// Inbound address prefixes (IPv4/IPv6)
    pub address_prefixes: Option<Vec<String>>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Direction that specifies whether the access rules is inbound/outbound.
    pub direction: Option<String>,
    /// Outbound rules in email address format. This access rule type is currently unavailable for use.
    pub email_addresses: Option<Vec<String>>,
    /// Outbound rules in fully qualified domain name format.
    pub fully_qualified_domain_names: Option<Vec<String>>,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// Rule specified by the perimeter id.
    pub network_security_perimeters: Vec<network::v20250101::PerimeterBasedAccessRuleResponse>,
    /// Outbound rules in phone number format. This access rule type is currently unavailable for use.
    pub phone_numbers: Option<Vec<String>>,
    /// The provisioning state of the scope assignment resource.
    pub provisioning_state: String,
    /// Inbound rules of type service tag. This access rule type is currently unavailable for use.
    pub service_tags: Option<Vec<String>>,
    /// List of subscription ids
    pub subscriptions: Option<Vec<network::v20250101::SubscriptionIdResponse>>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::v20250101::SystemDataResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the specified NSP access rule by name.
pub async fn get_network_security_perimeter_access_rule(
    ctx: &Context,
    args: GetNetworkSecurityPerimeterAccessRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkSecurityPerimeterAccessRuleResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("accessRuleName".to_string(), json!(args.access_rule_name));
    invoke_args.insert("networkSecurityPerimeterName".to_string(), json!(args.network_security_perimeter_name));
    invoke_args.insert("profileName".to_string(), json!(args.profile_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250101:getNetworkSecurityPerimeterAccessRule", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityPerimeterAccessRuleResult {
        address_prefixes: result.fields.get("addressPrefixes").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        direction: result.fields.get("direction").cloned().map(serde_json::from_value).transpose()?,
        email_addresses: result.fields.get("emailAddresses").cloned().map(serde_json::from_value).transpose()?,
        fully_qualified_domain_names: result.fields.get("fullyQualifiedDomainNames").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        network_security_perimeters: serde_json::from_value(result.fields.get("networkSecurityPerimeters").cloned().unwrap_or_default())?
            ,
        phone_numbers: result.fields.get("phoneNumbers").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        service_tags: result.fields.get("serviceTags").cloned().map(serde_json::from_value).transpose()?,
        subscriptions: result.fields.get("subscriptions").cloned().map(serde_json::from_value).transpose()?,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
