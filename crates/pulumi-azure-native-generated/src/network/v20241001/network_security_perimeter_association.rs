use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The NSP resource association resource
pub struct NetworkSecurityPerimeterAssociationArgs {
    /// Access mode on the association.
    pub access_mode: Option<Input<serde_json::Value>>,
    /// The name of the NSP association.
    pub association_name: Option<Input<String>>,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: Input<String>,
    /// The PaaS resource to be associated.
    pub private_link_resource: Option<Input<network::v20241001::SubResourceArgs>>,
    /// Profile id to which the PaaS resource is associated.
    pub profile: Option<Input<network::v20241001::SubResourceArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// The NSP resource association resource
pub struct NetworkSecurityPerimeterAssociation {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Access mode on the association.
    pub access_mode: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Specifies if there are provisioning issues
    pub has_provisioning_issues: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// The PaaS resource to be associated.
    pub private_link_resource: Output<serde_json::Value>,
    /// Profile id to which the PaaS resource is associated.
    pub profile: Output<serde_json::Value>,
    /// The provisioning state of the resource  association resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
}

impl NetworkSecurityPerimeterAssociation {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20241001:NetworkSecurityPerimeterAssociation";

    /// Create a new NetworkSecurityPerimeterAssociation resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkSecurityPerimeterAssociationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.access_mode {
            pulumi_sdk::resolve_input("accessMode", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.association_name {
            pulumi_sdk::resolve_input("associationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkSecurityPerimeterName", args.network_security_perimeter_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.private_link_resource {
            pulumi_sdk::resolve_input("privateLinkResource", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.profile {
            pulumi_sdk::resolve_input("profile", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            access_mode: registered.outputs.get("accessMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            has_provisioning_issues: registered.outputs.get("hasProvisioningIssues")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_link_resource: registered.outputs.get("privateLinkResource")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            profile: registered.outputs.get("profile")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
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
