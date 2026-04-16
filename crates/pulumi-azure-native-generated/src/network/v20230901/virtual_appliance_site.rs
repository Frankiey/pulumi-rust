use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Virtual Appliance Site resource.
pub struct VirtualApplianceSiteArgs {
    /// Address Prefix.
    pub address_prefix: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Name of the virtual appliance site.
    pub name: Option<Input<String>>,
    /// The name of the Network Virtual Appliance.
    pub network_virtual_appliance_name: Input<String>,
    /// Office 365 Policy.
    pub o365policy: Option<Input<network::v20230901::Office365PolicyPropertiesArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The name of the site.
    pub site_name: Option<Input<String>>,
}

/// Virtual Appliance Site resource.
pub struct VirtualApplianceSite {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Address Prefix.
    pub address_prefix: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Name of the virtual appliance site.
    pub name: Output<serde_json::Value>,
    /// Office 365 Policy.
    pub o365policy: Output<serde_json::Value>,
    /// The provisioning state of the resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Site type.
    pub type_: Output<serde_json::Value>,
}

impl VirtualApplianceSite {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230901:VirtualApplianceSite";

    /// Create a new VirtualApplianceSite resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualApplianceSiteArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.address_prefix {
            pulumi_sdk::resolve_input("addressPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkVirtualApplianceName", args.network_virtual_appliance_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.o365policy {
            pulumi_sdk::resolve_input("o365Policy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.site_name {
            pulumi_sdk::resolve_input("siteName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            address_prefix: registered.outputs.get("addressPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            o365policy: registered.outputs.get("o365Policy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
