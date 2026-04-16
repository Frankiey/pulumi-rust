use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// ExpressRouteCircuit resource.
pub struct ExpressRouteCircuitArgs {
    /// Allow classic operations.
    pub allow_classic_operations: Option<Input<bool>>,
    /// The authorizationKey.
    pub authorization_key: Option<Input<String>>,
    /// The list of authorizations.
    pub authorizations: Option<Vec<Input<network::v20220901::ExpressRouteCircuitAuthorizationArgs>>>,
    /// The bandwidth of the circuit when the circuit is provisioned on an ExpressRoutePort resource.
    pub bandwidth_in_gbps: Option<Input<f64>>,
    /// The name of the circuit.
    pub circuit_name: Option<Input<String>>,
    /// The CircuitProvisioningState state of the resource.
    pub circuit_provisioning_state: Option<Input<String>>,
    /// The reference to the ExpressRoutePort resource when the circuit is provisioned on an ExpressRoutePort resource.
    pub express_route_port: Option<Input<network::v20220901::SubResourceArgs>>,
    /// The GatewayManager Etag.
    pub gateway_manager_etag: Option<Input<String>>,
    /// Flag denoting global reach status.
    pub global_reach_enabled: Option<Input<bool>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The list of peerings.
    pub peerings: Option<Vec<Input<network::v20220901::ExpressRouteCircuitPeeringArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The ServiceKey.
    pub service_key: Option<Input<String>>,
    /// The ServiceProviderNotes.
    pub service_provider_notes: Option<Input<String>>,
    /// The ServiceProviderProperties.
    pub service_provider_properties: Option<Input<network::v20220901::ExpressRouteCircuitServiceProviderPropertiesArgs>>,
    /// The ServiceProviderProvisioningState state of the resource.
    pub service_provider_provisioning_state: Option<Input<serde_json::Value>>,
    /// The SKU.
    pub sku: Option<Input<network::v20220901::ExpressRouteCircuitSkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// ExpressRouteCircuit resource.
pub struct ExpressRouteCircuit {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// Allow classic operations.
    pub allow_classic_operations: Output<serde_json::Value>,
    /// The authorizationKey.
    pub authorization_key: Output<serde_json::Value>,
    /// The authorization status of the Circuit.
    pub authorization_status: Output<serde_json::Value>,
    /// The list of authorizations.
    pub authorizations: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The bandwidth of the circuit when the circuit is provisioned on an ExpressRoutePort resource.
    pub bandwidth_in_gbps: Output<serde_json::Value>,
    /// The CircuitProvisioningState state of the resource.
    pub circuit_provisioning_state: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The reference to the ExpressRoutePort resource when the circuit is provisioned on an ExpressRoutePort resource.
    pub express_route_port: Output<serde_json::Value>,
    /// The GatewayManager Etag.
    pub gateway_manager_etag: Output<serde_json::Value>,
    /// Flag denoting global reach status.
    pub global_reach_enabled: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The list of peerings.
    pub peerings: Output<serde_json::Value>,
    /// The provisioning state of the express route circuit resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The ServiceKey.
    pub service_key: Output<serde_json::Value>,
    /// The ServiceProviderNotes.
    pub service_provider_notes: Output<serde_json::Value>,
    /// The ServiceProviderProperties.
    pub service_provider_properties: Output<serde_json::Value>,
    /// The ServiceProviderProvisioningState state of the resource.
    pub service_provider_provisioning_state: Output<serde_json::Value>,
    /// The SKU.
    pub sku: Output<serde_json::Value>,
    /// The identifier of the circuit traffic. Outer tag for QinQ encapsulation.
    pub stag: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl ExpressRouteCircuit {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220901:ExpressRouteCircuit";

    /// Create a new ExpressRouteCircuit resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ExpressRouteCircuitArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.allow_classic_operations {
            pulumi_sdk::resolve_input("allowClassicOperations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.authorization_key {
            pulumi_sdk::resolve_input("authorizationKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.authorizations {
            pulumi_sdk::resolve_input_vec("authorizations", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.bandwidth_in_gbps {
            pulumi_sdk::resolve_input("bandwidthInGbps", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.circuit_name {
            pulumi_sdk::resolve_input("circuitName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.circuit_provisioning_state {
            pulumi_sdk::resolve_input("circuitProvisioningState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.express_route_port {
            pulumi_sdk::resolve_input("expressRoutePort", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.gateway_manager_etag {
            pulumi_sdk::resolve_input("gatewayManagerEtag", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.global_reach_enabled {
            pulumi_sdk::resolve_input("globalReachEnabled", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.peerings {
            pulumi_sdk::resolve_input_vec("peerings", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.service_key {
            pulumi_sdk::resolve_input("serviceKey", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_provider_notes {
            pulumi_sdk::resolve_input("serviceProviderNotes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_provider_properties {
            pulumi_sdk::resolve_input("serviceProviderProperties", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_provider_provisioning_state {
            pulumi_sdk::resolve_input("serviceProviderProvisioningState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
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
            allow_classic_operations: registered.outputs.get("allowClassicOperations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            authorization_key: registered.outputs.get("authorizationKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            authorization_status: registered.outputs.get("authorizationStatus")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            authorizations: registered.outputs.get("authorizations")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            bandwidth_in_gbps: registered.outputs.get("bandwidthInGbps")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            circuit_provisioning_state: registered.outputs.get("circuitProvisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            express_route_port: registered.outputs.get("expressRoutePort")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            gateway_manager_etag: registered.outputs.get("gatewayManagerEtag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            global_reach_enabled: registered.outputs.get("globalReachEnabled")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            peerings: registered.outputs.get("peerings")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_key: registered.outputs.get("serviceKey")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_provider_notes: registered.outputs.get("serviceProviderNotes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_provider_properties: registered.outputs.get("serviceProviderProperties")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_provider_provisioning_state: registered.outputs.get("serviceProviderProvisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            stag: registered.outputs.get("stag")
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
