use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Retrieves the boot diagnostic logs for a VM instance belonging to the specified Network Virtual Appliance.
#[derive(Default)]
pub struct GetNetworkVirtualApplianceBootDiagnosticLogsArgs {
    /// Specifies the sas-url to the storage blob into which console screen shot for the requested instance will be written
    pub console_screenshot_storage_sas_url: Option<String>,
    /// The network virtual appliance instance id for which boot diagnostic logs is being requested
    pub instance_id: Option<i64>,
    /// The name of Network Virtual Appliance.
    pub network_virtual_appliance_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// Specifies the sas-url to the storage blob into which serial console logs for the requested instance will be written
    pub serial_console_storage_sas_url: Option<String>,
}

/// Result of the function invocation.
pub struct GetNetworkVirtualApplianceBootDiagnosticLogsResult {
    /// The network virtual appliance instance id for which boot diagnostic logs is being requested
    pub instance_id: Option<i64>,
}

/// Retrieves the boot diagnostic logs for a VM instance belonging to the specified Network Virtual Appliance.
pub async fn get_network_virtual_appliance_boot_diagnostic_logs(
    ctx: &Context,
    args: GetNetworkVirtualApplianceBootDiagnosticLogsArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkVirtualApplianceBootDiagnosticLogsResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.console_screenshot_storage_sas_url {
        invoke_args.insert("consoleScreenshotStorageSasUrl".to_string(), json!(v));
    }
    if let Some(v) = args.instance_id {
        invoke_args.insert("instanceId".to_string(), json!(v));
    }
    invoke_args.insert("networkVirtualApplianceName".to_string(), json!(args.network_virtual_appliance_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    if let Some(v) = args.serial_console_storage_sas_url {
        invoke_args.insert("serialConsoleStorageSasUrl".to_string(), json!(v));
    }

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250501:getNetworkVirtualApplianceBootDiagnosticLogs", invoke_args, &opts).await?;

    Ok(GetNetworkVirtualApplianceBootDiagnosticLogsResult {
        instance_id: result.fields.get("instanceId").cloned().map(serde_json::from_value).transpose()?,
    })
}
