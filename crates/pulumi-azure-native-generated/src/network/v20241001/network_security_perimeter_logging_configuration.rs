use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The NSP logging configuration
pub struct NetworkSecurityPerimeterLoggingConfigurationArgs {
    /// The log categories to enable in the NSP logging configuration.
    pub enabled_log_categories: Option<Vec<Input<String>>>,
    /// The name of the NSP logging configuration. Accepts 'instance' as name.
    pub logging_configuration_name: Option<Input<String>>,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The version of the NSP logging configuration.
    pub version: Option<Input<String>>,
}

/// The NSP logging configuration
pub struct NetworkSecurityPerimeterLoggingConfiguration {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The log categories to enable in the NSP logging configuration.
    pub enabled_log_categories: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
    /// The version of the NSP logging configuration.
    pub version: Output<serde_json::Value>,
}

impl NetworkSecurityPerimeterLoggingConfiguration {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20241001:NetworkSecurityPerimeterLoggingConfiguration";

    /// Create a new NetworkSecurityPerimeterLoggingConfiguration resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkSecurityPerimeterLoggingConfigurationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.enabled_log_categories {
            pulumi_sdk::resolve_input_vec("enabledLogCategories", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.logging_configuration_name {
            pulumi_sdk::resolve_input("loggingConfigurationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkSecurityPerimeterName", args.network_security_perimeter_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.version {
            pulumi_sdk::resolve_input("version", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            enabled_log_categories: registered.outputs.get("enabledLogCategories")
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
            version: registered.outputs.get("version")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
