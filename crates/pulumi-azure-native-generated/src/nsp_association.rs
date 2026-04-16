use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The NSP resource association resource
/// 
/// Uses Azure REST API version 2023-08-01-preview. In version 2.x of the Azure Native provider, it used API version 2021-02-01-preview.
/// 
/// Other available API versions: 2021-02-01-preview, 2023-07-01-preview. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct NspAssociationArgs {
    /// Access mode on the association.
    pub access_mode: Option<Input<serde_json::Value>>,
    /// The name of the NSP association.
    pub association_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: Input<String>,
    /// The PaaS resource to be associated.
    pub private_link_resource: Option<Input<network::SubResourceArgs>>,
    /// Profile id to which the PaaS resource is associated.
    pub profile: Option<Input<network::SubResourceArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// The NSP resource association resource
/// 
/// Uses Azure REST API version 2023-08-01-preview. In version 2.x of the Azure Native provider, it used API version 2021-02-01-preview.
/// 
/// Other available API versions: 2021-02-01-preview, 2023-07-01-preview. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct NspAssociation {
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
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The PaaS resource to be associated.
    pub private_link_resource: Output<serde_json::Value>,
    /// Profile id to which the PaaS resource is associated.
    pub profile: Output<serde_json::Value>,
    /// The provisioning state of the resource  association resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl NspAssociation {
    const TYPE_TOKEN: &'static str = "azure-native:network:NspAssociation";

    /// Create a new NspAssociation resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NspAssociationArgs,
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
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkSecurityPerimeterName", args.network_security_perimeter_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.private_link_resource {
            pulumi_sdk::resolve_input("privateLinkResource", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.profile {
            pulumi_sdk::resolve_input("profile", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            access_mode: registered.outputs.get("accessMode")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            has_provisioning_issues: registered.outputs.get("hasProvisioningIssues")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
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
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
