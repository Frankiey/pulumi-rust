use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// ExpressRoutePort resource definition.
pub struct ExpressRoutePortArgs {
    /// Bandwidth of procured ports in Gbps.
    pub bandwidth_in_gbps: Option<Input<i64>>,
    /// The billing type of the ExpressRoutePort resource.
    pub billing_type: Option<Input<serde_json::Value>>,
    /// Encapsulation method on physical ports.
    pub encapsulation: Option<Input<serde_json::Value>>,
    /// The name of the ExpressRoutePort resource.
    pub express_route_port_name: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The identity of ExpressRoutePort, if configured.
    pub identity: Option<Input<network::v20250501::ManagedServiceIdentityArgs>>,
    /// The set of physical links of the ExpressRoutePort resource.
    pub links: Option<Vec<Input<network::v20250501::ExpressRouteLinkArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the peering location that the ExpressRoutePort is mapped to physically.
    pub peering_location: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// ExpressRoutePort resource definition.
pub struct ExpressRoutePort {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Date of the physical port allocation to be used in Letter of Authorization.
    pub allocation_date: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Bandwidth of procured ports in Gbps.
    pub bandwidth_in_gbps: Output<serde_json::Value>,
    /// The billing type of the ExpressRoutePort resource.
    pub billing_type: Output<serde_json::Value>,
    /// Reference the ExpressRoute circuit(s) that are provisioned on this ExpressRoutePort resource.
    pub circuits: Output<serde_json::Value>,
    /// Encapsulation method on physical ports.
    pub encapsulation: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Ether type of the physical port.
    pub ether_type: Output<serde_json::Value>,
    /// The identity of ExpressRoutePort, if configured.
    pub identity: Output<serde_json::Value>,
    /// The set of physical links of the ExpressRoutePort resource.
    pub links: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Maximum transmission unit of the physical port pair(s).
    pub mtu: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The name of the peering location that the ExpressRoutePort is mapped to physically.
    pub peering_location: Output<serde_json::Value>,
    /// Aggregate Gbps of associated circuit bandwidths.
    pub provisioned_bandwidth_in_gbps: Output<serde_json::Value>,
    /// The provisioning state of the express route port resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the express route port resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl ExpressRoutePort {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250501:ExpressRoutePort";

    /// Create a new ExpressRoutePort resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ExpressRoutePortArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.bandwidth_in_gbps {
            pulumi_sdk::resolve_input("bandwidthInGbps", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.billing_type {
            pulumi_sdk::resolve_input("billingType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.encapsulation {
            pulumi_sdk::resolve_input("encapsulation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_port_name {
            pulumi_sdk::resolve_input("expressRoutePortName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.links {
            pulumi_sdk::resolve_input_vec("links", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peering_location {
            pulumi_sdk::resolve_input("peeringLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            allocation_date: registered.outputs.get("allocationDate")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            bandwidth_in_gbps: registered.outputs.get("bandwidthInGbps")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            billing_type: registered.outputs.get("billingType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            circuits: registered.outputs.get("circuits")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            encapsulation: registered.outputs.get("encapsulation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ether_type: registered.outputs.get("etherType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            links: registered.outputs.get("links")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            mtu: registered.outputs.get("mtu")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peering_location: registered.outputs.get("peeringLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioned_bandwidth_in_gbps: registered.outputs.get("provisionedBandwidthInGbps")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
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
