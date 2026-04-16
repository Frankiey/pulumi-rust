use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets information about the specified DDoS protection plan.
/// 
/// Uses Azure REST API version 2024-05-01.
#[derive(Default)]
pub struct GetDdosProtectionPlanArgs {
    /// The name of the DDoS protection plan.
    pub ddos_protection_plan_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetDdosProtectionPlanResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the DDoS protection plan resource.
    pub provisioning_state: String,
    /// The list of public IPs associated with the DDoS protection plan resource. This list is read-only.
    pub public_ip_addresses: Vec<network::SubResourceResponse>,
    /// The resource GUID property of the DDoS protection plan resource. It uniquely identifies the resource, even if the user changes its name or migrate the resource across subscriptions or resource groups.
    pub resource_guid: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
    /// The list of virtual networks associated with the DDoS protection plan resource. This list is read-only.
    pub virtual_networks: Vec<network::SubResourceResponse>,
}

/// Gets information about the specified DDoS protection plan.
pub async fn get_ddos_protection_plan(
    ctx: &Context,
    args: GetDdosProtectionPlanArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetDdosProtectionPlanResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("ddosProtectionPlanName".to_string(), json!(args.ddos_protection_plan_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:getDdosProtectionPlan", invoke_args, &opts).await?;

    Ok(GetDdosProtectionPlanResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        public_ip_addresses: serde_json::from_value(result.fields.get("publicIPAddresses").cloned().unwrap_or_default())?
            ,
        resource_guid: serde_json::from_value(result.fields.get("resourceGuid").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
        virtual_networks: serde_json::from_value(result.fields.get("virtualNetworks").cloned().unwrap_or_default())?
            ,
    })
}
