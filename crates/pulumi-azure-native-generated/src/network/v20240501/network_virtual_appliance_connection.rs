use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// NetworkVirtualApplianceConnection resource.
pub struct NetworkVirtualApplianceConnectionArgs {
    /// The name of the NVA connection.
    pub connection_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource.
    pub name: Option<Input<String>>,
    /// The name of the Network Virtual Appliance.
    pub network_virtual_appliance_name: Input<String>,
    /// Properties of the express route connection.
    pub properties: Option<Input<network::v20240501::NetworkVirtualApplianceConnectionPropertiesArgs>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// NetworkVirtualApplianceConnection resource.
pub struct NetworkVirtualApplianceConnection {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The name of the resource.
    pub name: Output<serde_json::Value>,
    /// Properties of the express route connection.
    pub properties: Output<serde_json::Value>,
}

impl NetworkVirtualApplianceConnection {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240501:NetworkVirtualApplianceConnection";

    /// Create a new NetworkVirtualApplianceConnection resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkVirtualApplianceConnectionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.connection_name {
            pulumi_sdk::resolve_input("connectionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkVirtualApplianceName", args.network_virtual_appliance_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.properties {
            pulumi_sdk::resolve_input("properties", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            properties: registered.outputs.get("properties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
