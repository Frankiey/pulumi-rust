use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// ExpressRoute gateway resource.
pub struct ExpressRouteGatewayArgs {
    /// Configures this gateway to accept traffic from non Virtual WAN networks.
    pub allow_non_virtual_wan_traffic: Option<Input<bool>>,
    /// Configuration for auto scaling.
    pub auto_scale_configuration: Option<Input<network::v20221101::ExpressRouteGatewayPropertiesAutoScaleConfigurationArgs>>,
    /// List of ExpressRoute connections to the ExpressRoute gateway.
    pub express_route_connections: Option<Vec<Input<network::v20221101::ExpressRouteConnectionArgs>>>,
    /// The name of the ExpressRoute gateway.
    pub express_route_gateway_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The Virtual Hub where the ExpressRoute gateway is or will be deployed.
    pub virtual_hub: Input<network::v20221101::VirtualHubIdArgs>,
}

/// ExpressRoute gateway resource.
pub struct ExpressRouteGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Configures this gateway to accept traffic from non Virtual WAN networks.
    pub allow_non_virtual_wan_traffic: Output<serde_json::Value>,
    /// Configuration for auto scaling.
    pub auto_scale_configuration: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// List of ExpressRoute connections to the ExpressRoute gateway.
    pub express_route_connections: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the express route gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The Virtual Hub where the ExpressRoute gateway is or will be deployed.
    pub virtual_hub: Output<serde_json::Value>,
}

impl ExpressRouteGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20221101:ExpressRouteGateway";

    /// Create a new ExpressRouteGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ExpressRouteGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.allow_non_virtual_wan_traffic {
            pulumi_sdk::resolve_input("allowNonVirtualWanTraffic", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.auto_scale_configuration {
            pulumi_sdk::resolve_input("autoScaleConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_connections {
            pulumi_sdk::resolve_input_vec("expressRouteConnections", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_gateway_name {
            pulumi_sdk::resolve_input("expressRouteGatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualHub", args.virtual_hub, &mut inputs, &mut deps, &mut prop_deps).await;

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
            allow_non_virtual_wan_traffic: registered.outputs.get("allowNonVirtualWanTraffic")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            auto_scale_configuration: registered.outputs.get("autoScaleConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_connections: registered.outputs.get("expressRouteConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
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
            virtual_hub: registered.outputs.get("virtualHub")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
