use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The network security perimeter profile resource
pub struct NetworkSecurityPerimeterProfileArgs {
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: Input<String>,
    /// The name of the NSP profile.
    pub profile_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// The network security perimeter profile resource
pub struct NetworkSecurityPerimeterProfile {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Version number that increases with every update to access rules within the profile.
    pub access_rules_version: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Version number that increases with every update to diagnostic settings within the profile.
    pub diagnostic_settings_version: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl NetworkSecurityPerimeterProfile {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240701:NetworkSecurityPerimeterProfile";

    /// Create a new NetworkSecurityPerimeterProfile resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkSecurityPerimeterProfileArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("networkSecurityPerimeterName", args.network_security_perimeter_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.profile_name {
            pulumi_sdk::resolve_input("profileName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            access_rules_version: registered.outputs.get("accessRulesVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            diagnostic_settings_version: registered.outputs.get("diagnosticSettingsVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
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
