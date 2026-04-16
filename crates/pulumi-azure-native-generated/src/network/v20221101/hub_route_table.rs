use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// RouteTable resource in a virtual hub.
pub struct HubRouteTableArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// List of labels associated with this route table.
    pub labels: Option<Vec<Input<String>>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: Input<String>,
    /// The name of the RouteTable.
    pub route_table_name: Option<Input<String>>,
    /// List of all routes.
    pub routes: Option<Vec<Input<network::v20221101::HubRouteArgs>>>,
    /// The name of the VirtualHub.
    pub virtual_hub_name: Input<String>,
}

/// RouteTable resource in a virtual hub.
pub struct HubRouteTable {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// List of all connections associated with this route table.
    pub associated_connections: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// List of labels associated with this route table.
    pub labels: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// List of all connections that advertise to this route table.
    pub propagating_connections: Output<serde_json::Value>,
    /// The provisioning state of the RouteTable resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// List of all routes.
    pub routes: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl HubRouteTable {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20221101:HubRouteTable";

    /// Create a new HubRouteTable resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: HubRouteTableArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.labels {
            pulumi_sdk::resolve_input_vec("labels", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            associated_connections: registered.outputs.get("associatedConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            labels: registered.outputs.get("labels")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            propagating_connections: registered.outputs.get("propagatingConnections")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            routes: registered.outputs.get("routes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
