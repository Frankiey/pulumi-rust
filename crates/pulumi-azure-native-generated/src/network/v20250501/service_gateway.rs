use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// ServiceGateway resource.
pub struct ServiceGatewayArgs {
    /// The geo-location where the resource lives
    pub location: Option<Input<String>>,
    /// The name of the resource group. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// Route Target address of Service gateway
    pub route_target_address: Option<Input<network::v20250501::RouteTargetAddressPropertiesFormatArgs>>,
    /// Route Target address V6 of Service gateway
    pub route_target_address_v6: Option<Input<network::v20250501::RouteTargetAddressPropertiesFormatArgs>>,
    /// The name of the service gateway.
    pub service_gateway_name: Option<Input<String>>,
    /// The service gateway SKU.
    pub sku: Option<Input<network::v20250501::ServiceGatewaySkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// Reference to an existing virtual network.
    pub virtual_network: Option<Input<network::v20250501::VirtualNetworkArgs>>,
    /// A list of availability zones denoting the zone in which service gateway should be deployed. 
    pub zones: Option<Vec<Input<String>>>,
}

/// ServiceGateway resource.
pub struct ServiceGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The geo-location where the resource lives
    pub location: Output<serde_json::Value>,
    /// The name of the resource
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the service gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the service gateway resource.
    pub resource_guid: Output<serde_json::Value>,
    /// Route Target address of Service gateway
    pub route_target_address: Output<serde_json::Value>,
    /// Route Target address V6 of Service gateway
    pub route_target_address_v6: Output<serde_json::Value>,
    /// The service gateway SKU.
    pub sku: Output<serde_json::Value>,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: Output<serde_json::Value>,
    /// Reference to an existing virtual network.
    pub virtual_network: Output<serde_json::Value>,
    /// A list of availability zones denoting the zone in which service gateway should be deployed. 
    pub zones: Output<serde_json::Value>,
}

impl ServiceGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20250501:ServiceGateway";

    /// Create a new ServiceGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ServiceGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.route_target_address {
            pulumi_sdk::resolve_input("routeTargetAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.route_target_address_v6 {
            pulumi_sdk::resolve_input("routeTargetAddressV6", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_gateway_name {
            pulumi_sdk::resolve_input("serviceGatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.virtual_network {
            pulumi_sdk::resolve_input("virtualNetwork", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.zones {
            pulumi_sdk::resolve_input_vec("zones", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
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
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_target_address: registered.outputs.get("routeTargetAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            route_target_address_v6: registered.outputs.get("routeTargetAddressV6")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            virtual_network: registered.outputs.get("virtualNetwork")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
