use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// P2SVpnServerConfiguration Resource.
pub struct P2sVpnServerConfigurationArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the P2SVpnServerConfiguration.
    pub p2s_vpn_server_configuration_name: Option<Input<String>>,
    /// Properties of the P2SVpnServer configuration.
    pub properties: Option<Input<network::v20190601::P2SVpnServerConfigurationPropertiesArgs>>,
    /// The resource group name of the VirtualWan.
    pub resource_group_name: Input<String>,
    /// The name of the VirtualWan.
    pub virtual_wan_name: Input<String>,
}

/// P2SVpnServerConfiguration Resource.
pub struct P2sVpnServerConfiguration {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Gets a unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// Properties of the P2SVpnServer configuration.
    pub properties: Output<serde_json::Value>,
}

impl P2sVpnServerConfiguration {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20190601:P2sVpnServerConfiguration";

    /// Create a new P2sVpnServerConfiguration resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: P2sVpnServerConfigurationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.p2s_vpn_server_configuration_name {
            pulumi_sdk::resolve_input("p2SVpnServerConfigurationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.properties {
            pulumi_sdk::resolve_input("properties", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("virtualWanName", args.virtual_wan_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            etag: registered.outputs.get("etag")
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
