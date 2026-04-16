use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// VirtualHubRouteTableV2 Resource.
pub struct VirtualHubRouteTableV2Args {
    /// List of all connections attached to this route table v2.
    pub attached_connections: Option<Vec<Input<String>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: Input<String>,
    /// The name of the VirtualHubRouteTableV2.
    pub route_table_name: Option<Input<String>>,
    /// List of all routes.
    pub routes: Option<Vec<Input<network::v20240301::VirtualHubRouteV2Args>>>,
    /// The name of the VirtualHub.
    pub virtual_hub_name: Input<String>,
}

/// VirtualHubRouteTableV2 Resource.
pub struct VirtualHubRouteTableV2 {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// List of all connections attached to this route table v2.
    pub attached_connections: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the virtual hub route table v2 resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// List of all routes.
    pub routes: Output<serde_json::Value>,
}

impl VirtualHubRouteTableV2 {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20240301:VirtualHubRouteTableV2";

    /// Create a new VirtualHubRouteTableV2 resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualHubRouteTableV2Args,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.attached_connections {
            pulumi_sdk::resolve_input_vec("attachedConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.route_table_name {
            pulumi_sdk::resolve_input("routeTableName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.routes {
            pulumi_sdk::resolve_input_vec("routes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualHubName", args.virtual_hub_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            attached_connections: registered.outputs.get("attachedConnections")
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
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routes: registered.outputs.get("routes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
