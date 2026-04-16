use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// NetworkSecurityGroup resource.
pub struct NetworkSecurityGroupArgs {
    /// The default security rules of network security group.
    pub default_security_rules: Option<Vec<Input<network::v20190701::SecurityRuleArgs>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the network security group.
    pub network_security_group_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The resource GUID property of the network security group resource.
    pub resource_guid: Option<Input<String>>,
    /// A collection of security rules of the network security group.
    pub security_rules: Option<Vec<Input<network::v20190701::SecurityRuleArgs>>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// NetworkSecurityGroup resource.
pub struct NetworkSecurityGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The default security rules of network security group.
    pub default_security_rules: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// A collection of references to network interfaces.
    pub network_interfaces: Output<serde_json::Value>,
    /// The provisioning state of the network security group resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the network security group resource.
    pub resource_guid: Output<serde_json::Value>,
    /// A collection of security rules of the network security group.
    pub security_rules: Output<serde_json::Value>,
    /// A collection of references to subnets.
    pub subnets: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl NetworkSecurityGroup {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190701:NetworkSecurityGroup";

    /// Create a new NetworkSecurityGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkSecurityGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.default_security_rules {
            pulumi_sdk::resolve_input_vec("defaultSecurityRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_security_group_name {
            pulumi_sdk::resolve_input("networkSecurityGroupName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.resource_guid {
            pulumi_sdk::resolve_input("resourceGuid", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.security_rules {
            pulumi_sdk::resolve_input_vec("securityRules", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            default_security_rules: registered.outputs.get("defaultSecurityRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            network_interfaces: registered.outputs.get("networkInterfaces")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            security_rules: registered.outputs.get("securityRules")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            subnets: registered.outputs.get("subnets")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
