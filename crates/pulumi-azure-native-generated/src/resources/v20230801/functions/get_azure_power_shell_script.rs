use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets a deployment script with a given name.
#[derive(Default)]
pub struct GetAzurePowerShellScriptArgs {
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: String,
    /// Name of the deployment script.
    pub script_name: String,
}

/// Result of the function invocation.
pub struct GetAzurePowerShellScriptResult {
    /// Command line arguments to pass to the script. Arguments are separated by spaces. ex: -Name blue* -Location 'West US 2' 
    pub arguments: Option<String>,
    /// Azure PowerShell module version to be used.
    pub az_power_shell_version: String,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// The clean up preference when the script execution gets in a terminal state. Default setting is 'Always'.
    pub cleanup_preference: Option<String>,
    /// Container settings.
    pub container_settings: Option<resources::v20230801::ContainerConfigurationResponse>,
    /// The environment variables to pass over to the script.
    pub environment_variables: Option<Vec<resources::v20230801::EnvironmentVariableResponse>>,
    /// Gets or sets how the deployment script should be forced to execute even if the script resource has not changed. Can be current time stamp or a GUID.
    pub force_update_tag: Option<String>,
    /// String Id used to locate any resource on Azure.
    pub id: String,
    /// Optional property. Managed identity to be used for this deployment script. Currently, only user-assigned MSI is supported.
    pub identity: Option<resources::v20230801::ManagedServiceIdentityResponse>,
    /// Type of the script.
    pub kind: String,
    /// The location of the ACI and the storage account for the deployment script.
    pub location: String,
    /// Name of this resource.
    pub name: String,
    /// List of script outputs.
    pub outputs: HashMap<String, serde_json::Value>,
    /// Uri for the script. This is the entry point for the external script.
    pub primary_script_uri: Option<String>,
    /// State of the script execution. This only appears in the response.
    pub provisioning_state: String,
    /// Interval for which the service retains the script resource after it reaches a terminal state. Resource will be deleted when this duration expires. Duration is based on ISO 8601 pattern (for example P1D means one day).
    pub retention_interval: String,
    /// Script body.
    pub script_content: Option<String>,
    /// Contains the results of script execution.
    pub status: resources::v20230801::ScriptStatusResponse,
    /// Storage Account settings.
    pub storage_account_settings: Option<resources::v20230801::StorageAccountConfigurationResponse>,
    /// Supporting files for the external script.
    pub supporting_script_uris: Option<Vec<String>>,
    /// The system metadata related to this resource.
    pub system_data: resources::v20230801::SystemDataResponse,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Maximum allowed script execution time specified in ISO 8601 format. Default value is P1D
    pub timeout: Option<String>,
    /// Type of this resource.
    pub type_: String,
}

/// Gets a deployment script with a given name.
pub async fn get_azure_power_shell_script(
    ctx: &Context,
    args: GetAzurePowerShellScriptArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetAzurePowerShellScriptResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("scriptName".to_string(), json!(args.script_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:resources/v20230801:getAzurePowerShellScript", invoke_args, &opts).await?;

    Ok(GetAzurePowerShellScriptResult {
        arguments: result.fields.get("arguments").cloned().map(serde_json::from_value).transpose()?,
        az_power_shell_version: serde_json::from_value(result.fields.get("azPowerShellVersion").cloned().unwrap_or_default())?
            ,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        cleanup_preference: result.fields.get("cleanupPreference").cloned().map(serde_json::from_value).transpose()?,
        container_settings: result.fields.get("containerSettings").cloned().map(serde_json::from_value).transpose()?,
        environment_variables: result.fields.get("environmentVariables").cloned().map(serde_json::from_value).transpose()?,
        force_update_tag: result.fields.get("forceUpdateTag").cloned().map(serde_json::from_value).transpose()?,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        identity: result.fields.get("identity").cloned().map(serde_json::from_value).transpose()?,
        kind: serde_json::from_value(result.fields.get("kind").cloned().unwrap_or_default())?
            ,
        location: serde_json::from_value(result.fields.get("location").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        outputs: serde_json::from_value(result.fields.get("outputs").cloned().unwrap_or_default())?
            ,
        primary_script_uri: result.fields.get("primaryScriptUri").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        retention_interval: serde_json::from_value(result.fields.get("retentionInterval").cloned().unwrap_or_default())?
            ,
        script_content: result.fields.get("scriptContent").cloned().map(serde_json::from_value).transpose()?,
        status: serde_json::from_value(result.fields.get("status").cloned().unwrap_or_default())?
            ,
        storage_account_settings: result.fields.get("storageAccountSettings").cloned().map(serde_json::from_value).transpose()?,
        supporting_script_uris: result.fields.get("supportingScriptUris").cloned().map(serde_json::from_value).transpose()?,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        timeout: result.fields.get("timeout").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
