use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Object model for the Azure CLI script.
pub struct AzureCliScriptArgs {
    /// Command line arguments to pass to the script. Arguments are separated by spaces. ex: -Name blue* -Location 'West US 2' 
    pub arguments: Option<Input<String>>,
    /// Azure CLI module version to be used.
    pub az_cli_version: Input<String>,
    /// The clean up preference when the script execution gets in a terminal state. Default setting is 'Always'.
    pub cleanup_preference: Option<Input<serde_json::Value>>,
    /// Container settings.
    pub container_settings: Option<Input<resources::v20230801::ContainerConfigurationArgs>>,
    /// The environment variables to pass over to the script.
    pub environment_variables: Option<Vec<Input<resources::v20230801::EnvironmentVariableArgs>>>,
    /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID.
    pub force_update_tag: Option<Input<String>>,
    /// Optional property. Managed identity to be used for this deployment script. Currently, only user-assigned MSI is supported.
    pub identity: Option<Input<resources::v20230801::ManagedServiceIdentityArgs>>,
    /// Type of the script.
    pub kind: Input<String>,
    /// The location of the ACI and the storage account for the deployment script.
    pub location: Option<Input<String>>,
    /// Uri for the script. This is the entry point for the external script.
    pub primary_script_uri: Option<Input<String>>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// Interval for which the service retains the script resource after it reaches a terminal state. Resource will be deleted when this duration expires. Duration is based on ISO 8601 pattern (for example P1D means one day).
    pub retention_interval: Input<String>,
    /// Script body.
    pub script_content: Option<Input<String>>,
    /// Name of the deployment script.
    pub script_name: Option<Input<String>>,
    /// Storage Account settings.
    pub storage_account_settings: Option<Input<resources::v20230801::StorageAccountConfigurationArgs>>,
    /// Supporting files for the external script.
    pub supporting_script_uris: Option<Vec<Input<String>>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Maximum allowed script execution time specified in ISO 8601 format. Default value is P1D
    pub timeout: Option<Input<String>>,
}

/// Object model for the Azure CLI script.
pub struct AzureCliScript {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Command line arguments to pass to the script. Arguments are separated by spaces. ex: -Name blue* -Location 'West US 2' 
    pub arguments: Output<serde_json::Value>,
    /// Azure CLI module version to be used.
    pub az_cli_version: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The clean up preference when the script execution gets in a terminal state. Default setting is 'Always'.
    pub cleanup_preference: Output<serde_json::Value>,
    /// Container settings.
    pub container_settings: Output<serde_json::Value>,
    /// The environment variables to pass over to the script.
    pub environment_variables: Output<serde_json::Value>,
    /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID.
    pub force_update_tag: Output<serde_json::Value>,
    /// Optional property. Managed identity to be used for this deployment script. Currently, only user-assigned MSI is supported.
    pub identity: Output<serde_json::Value>,
    /// Type of the script.
    pub kind: Output<serde_json::Value>,
    /// The location of the ACI and the storage account for the deployment script.
    pub location: Output<serde_json::Value>,
    /// Name of this resource.
    pub name: Output<serde_json::Value>,
    /// List of script outputs.
    pub outputs: Output<serde_json::Value>,
    /// Uri for the script. This is the entry point for the external script.
    pub primary_script_uri: Output<serde_json::Value>,
    /// State of the script execution. This only appears in the response.
    pub provisioning_state: Output<serde_json::Value>,
    /// Interval for which the service retains the script resource after it reaches a terminal state. Resource will be deleted when this duration expires. Duration is based on ISO 8601 pattern (for example P1D means one day).
    pub retention_interval: Output<serde_json::Value>,
    /// Script body.
    pub script_content: Output<serde_json::Value>,
    /// Contains the results of script execution.
    pub status: Output<serde_json::Value>,
    /// Storage Account settings.
    pub storage_account_settings: Output<serde_json::Value>,
    /// Supporting files for the external script.
    pub supporting_script_uris: Output<serde_json::Value>,
    /// The system metadata related to this resource.
    pub system_data: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Maximum allowed script execution time specified in ISO 8601 format. Default value is P1D
    pub timeout: Output<serde_json::Value>,
    /// Type of this resource.
    pub type_: Output<serde_json::Value>,
}

impl AzureCliScript {
    const TYPE_TOKEN: &'static str = "azure-native:resources/v20230801:AzureCliScript";

    /// Create a new AzureCliScript resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: AzureCliScriptArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.arguments {
            pulumi_sdk::resolve_input("arguments", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("azCliVersion", args.az_cli_version, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.cleanup_preference {
            pulumi_sdk::resolve_input("cleanupPreference", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.container_settings {
            pulumi_sdk::resolve_input("containerSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.environment_variables {
            pulumi_sdk::resolve_input_vec("environmentVariables", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.force_update_tag {
            pulumi_sdk::resolve_input("forceUpdateTag", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("kind", args.kind, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.primary_script_uri {
            pulumi_sdk::resolve_input("primaryScriptUri", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("retentionInterval", args.retention_interval, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.script_content {
            pulumi_sdk::resolve_input("scriptContent", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.script_name {
            pulumi_sdk::resolve_input("scriptName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.storage_account_settings {
            pulumi_sdk::resolve_input("storageAccountSettings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.supporting_script_uris {
            pulumi_sdk::resolve_input_vec("supportingScriptUris", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.timeout {
            pulumi_sdk::resolve_input("timeout", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            arguments: registered.outputs.get("arguments")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            az_cli_version: registered.outputs.get("azCliVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            cleanup_preference: registered.outputs.get("cleanupPreference")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            container_settings: registered.outputs.get("containerSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            environment_variables: registered.outputs.get("environmentVariables")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            force_update_tag: registered.outputs.get("forceUpdateTag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            kind: registered.outputs.get("kind")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            outputs: registered.outputs.get("outputs")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            primary_script_uri: registered.outputs.get("primaryScriptUri")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            retention_interval: registered.outputs.get("retentionInterval")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            script_content: registered.outputs.get("scriptContent")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            status: registered.outputs.get("status")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            storage_account_settings: registered.outputs.get("storageAccountSettings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            supporting_script_uris: registered.outputs.get("supportingScriptUris")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            timeout: registered.outputs.get("timeout")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
