use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The network security perimeter link resource
pub struct NetworkSecurityPerimeterLinkArgs {
    /// Perimeter ARM Id for the remote NSP with which the link gets created in Auto-approval mode. It should be used when the NSP admin have Microsoft.Network/networkSecurityPerimeters/linkPerimeter/action permission on the remote NSP resource.
    pub auto_approved_remote_perimeter_resource_id: Option<Input<String>>,
    /// A message passed to the owner of the remote NSP link resource with this connection request. In case of Auto-approved flow, it is default to 'Auto Approved'. Restricted to 140 chars.
    pub description: Option<Input<String>>,
    /// The name of the NSP link.
    pub link_name: Option<Input<String>>,
    /// Local Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles.
    pub local_inbound_profiles: Option<Vec<Input<String>>>,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: Input<String>,
    /// Remote Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles. This property can only be updated in auto-approval mode.
    pub remote_inbound_profiles: Option<Vec<Input<String>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// The network security perimeter link resource
pub struct NetworkSecurityPerimeterLink {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Perimeter ARM Id for the remote NSP with which the link gets created in Auto-approval mode. It should be used when the NSP admin have Microsoft.Network/networkSecurityPerimeters/linkPerimeter/action permission on the remote NSP resource.
    pub auto_approved_remote_perimeter_resource_id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A message passed to the owner of the remote NSP link resource with this connection request. In case of Auto-approved flow, it is default to 'Auto Approved'. Restricted to 140 chars.
    pub description: Output<serde_json::Value>,
    /// Local Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles.
    pub local_inbound_profiles: Output<serde_json::Value>,
    /// Local Outbound profile names from which Outbound is allowed. In current version, it is readonly property and it's value is set to ['*'] to allow outbound from all profiles. In later version, user will be able to modify it.
    pub local_outbound_profiles: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the NSP Link resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Remote Inbound profile names to which Inbound is allowed. Use ['*'] to allow inbound to all profiles. This property can only be updated in auto-approval mode.
    pub remote_inbound_profiles: Output<serde_json::Value>,
    /// Remote Outbound profile names from which Outbound is allowed. In current version, it is readonly property and it's value is set to ['*'] to allow outbound from all profiles. In later version, user will be able to modify it.
    pub remote_outbound_profiles: Output<serde_json::Value>,
    /// Remote NSP Guid with which the link gets created.
    pub remote_perimeter_guid: Output<serde_json::Value>,
    /// Remote NSP location with which the link gets created.
    pub remote_perimeter_location: Output<serde_json::Value>,
    /// The NSP link state.
    pub status: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl NetworkSecurityPerimeterLink {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250501:NetworkSecurityPerimeterLink";

    /// Create a new NetworkSecurityPerimeterLink resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkSecurityPerimeterLinkArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.auto_approved_remote_perimeter_resource_id {
            pulumi_sdk::resolve_input("autoApprovedRemotePerimeterResourceId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.link_name {
            pulumi_sdk::resolve_input("linkName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.local_inbound_profiles {
            pulumi_sdk::resolve_input_vec("localInboundProfiles", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkSecurityPerimeterName", args.network_security_perimeter_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.remote_inbound_profiles {
            pulumi_sdk::resolve_input_vec("remoteInboundProfiles", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            auto_approved_remote_perimeter_resource_id: registered.outputs.get("autoApprovedRemotePerimeterResourceId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            local_inbound_profiles: registered.outputs.get("localInboundProfiles")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            local_outbound_profiles: registered.outputs.get("localOutboundProfiles")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_inbound_profiles: registered.outputs.get("remoteInboundProfiles")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_outbound_profiles: registered.outputs.get("remoteOutboundProfiles")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_perimeter_guid: registered.outputs.get("remotePerimeterGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            remote_perimeter_location: registered.outputs.get("remotePerimeterLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            status: registered.outputs.get("status")
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
