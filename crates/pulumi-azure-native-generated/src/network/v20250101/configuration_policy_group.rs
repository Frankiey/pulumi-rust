use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VpnServerConfigurationPolicyGroup Resource.
pub struct ConfigurationPolicyGroupArgs {
    /// The name of the ConfigurationPolicyGroup.
    pub configuration_policy_group_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Shows if this is a Default VpnServerConfigurationPolicyGroup or not.
    pub is_default: Option<Input<bool>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// Multiple PolicyMembers for VpnServerConfigurationPolicyGroup.
    pub policy_members: Option<Vec<Input<network::v20250101::VpnServerConfigurationPolicyGroupMemberArgs>>>,
    /// Priority for VpnServerConfigurationPolicyGroup.
    pub priority: Option<Input<i64>>,
    /// The resource group name of the ConfigurationPolicyGroup.
    pub resource_group_name: Input<String>,
    /// The name of the VpnServerConfiguration.
    pub vpn_server_configuration_name: Input<String>,
}

/// VpnServerConfigurationPolicyGroup Resource.
pub struct ConfigurationPolicyGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Shows if this is a Default VpnServerConfigurationPolicyGroup or not.
    pub is_default: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// List of references to P2SConnectionConfigurations.
    pub p2s_connection_configurations: Output<serde_json::Value>,
    /// Multiple PolicyMembers for VpnServerConfigurationPolicyGroup.
    pub policy_members: Output<serde_json::Value>,
    /// Priority for VpnServerConfigurationPolicyGroup.
    pub priority: Output<serde_json::Value>,
    /// The provisioning state of the VpnServerConfigurationPolicyGroup resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl ConfigurationPolicyGroup {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250101:ConfigurationPolicyGroup";

    /// Create a new ConfigurationPolicyGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ConfigurationPolicyGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.configuration_policy_group_name {
            pulumi_sdk::resolve_input("configurationPolicyGroupName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.is_default {
            pulumi_sdk::resolve_input("isDefault", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.policy_members {
            pulumi_sdk::resolve_input_vec("policyMembers", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.priority {
            pulumi_sdk::resolve_input("priority", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("vpnServerConfigurationName", args.vpn_server_configuration_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            is_default: registered.outputs.get("isDefault")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            p2s_connection_configurations: registered.outputs.get("p2SConnectionConfigurations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            policy_members: registered.outputs.get("policyMembers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            priority: registered.outputs.get("priority")
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
