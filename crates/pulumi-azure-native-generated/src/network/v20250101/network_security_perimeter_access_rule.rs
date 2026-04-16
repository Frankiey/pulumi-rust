use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The NSP access rule resource
pub struct NetworkSecurityPerimeterAccessRuleArgs {
    /// The name of the NSP access rule.
    pub access_rule_name: Option<Input<String>>,
    /// Inbound address prefixes (IPv4/IPv6)
    pub address_prefixes: Option<Vec<Input<String>>>,
    /// Direction that specifies whether the access rules is inbound/outbound.
    pub direction: Option<Input<serde_json::Value>>,
    /// Outbound rules in email address format. This access rule type is currently unavailable for use.
    pub email_addresses: Option<Vec<Input<String>>>,
    /// Outbound rules in fully qualified domain name format.
    pub fully_qualified_domain_names: Option<Vec<Input<String>>>,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: Input<String>,
    /// Outbound rules in phone number format. This access rule type is currently unavailable for use.
    pub phone_numbers: Option<Vec<Input<String>>>,
    /// The name of the NSP profile.
    pub profile_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Inbound rules of type service tag. This access rule type is currently unavailable for use.
    pub service_tags: Option<Vec<Input<String>>>,
    /// List of subscription ids
    pub subscriptions: Option<Vec<Input<network::v20250101::SubscriptionIdArgs>>>,
}

/// The NSP access rule resource
pub struct NetworkSecurityPerimeterAccessRule {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Inbound address prefixes (IPv4/IPv6)
    pub address_prefixes: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Direction that specifies whether the access rules is inbound/outbound.
    pub direction: Output<serde_json::Value>,
    /// Outbound rules in email address format. This access rule type is currently unavailable for use.
    pub email_addresses: Output<serde_json::Value>,
    /// Outbound rules in fully qualified domain name format.
    pub fully_qualified_domain_names: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// Rule specified by the perimeter id.
    pub network_security_perimeters: Output<serde_json::Value>,
    /// Outbound rules in phone number format. This access rule type is currently unavailable for use.
    pub phone_numbers: Output<serde_json::Value>,
    /// The provisioning state of the scope assignment resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Inbound rules of type service tag. This access rule type is currently unavailable for use.
    pub service_tags: Output<serde_json::Value>,
    /// List of subscription ids
    pub subscriptions: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl NetworkSecurityPerimeterAccessRule {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250101:NetworkSecurityPerimeterAccessRule";

    /// Create a new NetworkSecurityPerimeterAccessRule resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkSecurityPerimeterAccessRuleArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.access_rule_name {
            pulumi_sdk::resolve_input("accessRuleName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.address_prefixes {
            pulumi_sdk::resolve_input_vec("addressPrefixes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.direction {
            pulumi_sdk::resolve_input("direction", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.email_addresses {
            pulumi_sdk::resolve_input_vec("emailAddresses", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.fully_qualified_domain_names {
            pulumi_sdk::resolve_input_vec("fullyQualifiedDomainNames", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkSecurityPerimeterName", args.network_security_perimeter_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.phone_numbers {
            pulumi_sdk::resolve_input_vec("phoneNumbers", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("profileName", args.profile_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.service_tags {
            pulumi_sdk::resolve_input_vec("serviceTags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.subscriptions {
            pulumi_sdk::resolve_input_vec("subscriptions", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            address_prefixes: registered.outputs.get("addressPrefixes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            direction: registered.outputs.get("direction")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            email_addresses: registered.outputs.get("emailAddresses")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            fully_qualified_domain_names: registered.outputs.get("fullyQualifiedDomainNames")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_security_perimeters: registered.outputs.get("networkSecurityPerimeters")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            phone_numbers: registered.outputs.get("phoneNumbers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_tags: registered.outputs.get("serviceTags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            subscriptions: registered.outputs.get("subscriptions")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
