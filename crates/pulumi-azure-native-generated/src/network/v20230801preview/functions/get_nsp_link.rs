use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified NSP link resource.
#[derive(Default)]
pub struct GetNspLinkArgs {
    /// The name of the NSP link.
    pub link_name: String,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNspLinkResult {
    /// Perimeter ARM Id for the remote NSP with which the link gets created in Auto-approval mode. It should be used when the NSP admin have Microsoft.Network/networkSecurityPerimeters/linkPerimeter/action permission on the remote NSP resource.
    pub auto_approved_remote_perimeter_resource_id: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A message passed to the owner of the remote NSP link resource with this connection request. In case of Auto-approved flow, it is default to 'Auto Approved'. Restricted to 140 chars.
    pub description: Option<String>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// Resource ID.
    pub id: String,
    /// Local Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles.
    pub local_inbound_profiles: Option<Vec<String>>,
    /// Local Outbound profile names from which Outbound is allowed. In current version, it is readonly property and it's value is set to ['*'] to allow outbound from all profiles. In later version, user will be able to modify it.
    pub local_outbound_profiles: Vec<String>,
    /// Resource name.
    pub name: String,
    /// The provisioning state of the NSP Link resource.
    pub provisioning_state: String,
    /// Remote Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles. This property can only be updated in auto-approval mode.
    pub remote_inbound_profiles: Option<Vec<String>>,
    /// Remote Outbound profile names from which Outbound is allowed. In current version, it is readonly property and it's value is set to ['*'] to allow outbound from all profiles. In later version, user will be able to modify it.
    pub remote_outbound_profiles: Vec<String>,
    /// Remote NSP Guid with which the link gets created.
    pub remote_perimeter_guid: String,
    /// Remote NSP location with which the link gets created.
    pub remote_perimeter_location: String,
    /// The NSP link state.
    pub status: String,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified NSP link resource.
pub async fn get_nsp_link(
    ctx: &Context,
    args: GetNspLinkArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNspLinkResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("linkName".to_string(), json!(args.link_name));
    invoke_args.insert("networkSecurityPerimeterName".to_string(), json!(args.network_security_perimeter_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20230801preview:getNspLink", invoke_args, &opts).await?;

    Ok(GetNspLinkResult {
        auto_approved_remote_perimeter_resource_id: result.fields.get("autoApprovedRemotePerimeterResourceId").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        description: result.fields.get("description").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        local_inbound_profiles: result.fields.get("localInboundProfiles").cloned().map(serde_json::from_value).transpose()?,
        local_outbound_profiles: serde_json::from_value(result.fields.get("localOutboundProfiles").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        remote_inbound_profiles: result.fields.get("remoteInboundProfiles").cloned().map(serde_json::from_value).transpose()?,
        remote_outbound_profiles: serde_json::from_value(result.fields.get("remoteOutboundProfiles").cloned().unwrap_or_default())?
            ,
        remote_perimeter_guid: serde_json::from_value(result.fields.get("remotePerimeterGuid").cloned().unwrap_or_default())?
            ,
        remote_perimeter_location: serde_json::from_value(result.fields.get("remotePerimeterLocation").cloned().unwrap_or_default())?
            ,
        status: serde_json::from_value(result.fields.get("status").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
