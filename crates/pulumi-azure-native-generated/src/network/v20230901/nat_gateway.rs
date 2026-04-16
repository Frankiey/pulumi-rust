use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Nat Gateway resource.
pub struct NatGatewayArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The idle timeout of the nat gateway.
    pub idle_timeout_in_minutes: Option<Input<i64>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The name of the nat gateway.
    pub nat_gateway_name: Option<Input<String>>,
    /// An array of public ip addresses associated with the nat gateway resource.
    pub public_ip_addresses: Option<Vec<Input<network::v20230901::SubResourceArgs>>>,
    /// An array of public ip prefixes associated with the nat gateway resource.
    pub public_ip_prefixes: Option<Vec<Input<network::v20230901::SubResourceArgs>>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The nat gateway SKU.
    pub sku: Option<Input<network::v20230901::NatGatewaySkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// A list of availability zones denoting the zone in which Nat Gateway should be deployed.
    pub zones: Option<Vec<Input<String>>>,
}

/// Nat Gateway resource.
pub struct NatGateway {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The idle timeout of the nat gateway.
    pub idle_timeout_in_minutes: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the NAT gateway resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// An array of public ip addresses associated with the nat gateway resource.
    pub public_ip_addresses: Output<serde_json::Value>,
    /// An array of public ip prefixes associated with the nat gateway resource.
    pub public_ip_prefixes: Output<serde_json::Value>,
    /// The resource GUID property of the NAT gateway resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The nat gateway SKU.
    pub sku: Output<serde_json::Value>,
    /// An array of references to the subnets using this nat gateway resource.
    pub subnets: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// A list of availability zones denoting the zone in which Nat Gateway should be deployed.
    pub zones: Output<serde_json::Value>,
}

impl NatGateway {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230901:NatGateway";

    /// Create a new NatGateway resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NatGatewayArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.idle_timeout_in_minutes {
            pulumi_sdk::resolve_input("idleTimeoutInMinutes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nat_gateway_name {
            pulumi_sdk::resolve_input("natGatewayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_addresses {
            pulumi_sdk::resolve_input_vec("publicIpAddresses", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_prefixes {
            pulumi_sdk::resolve_input_vec("publicIpPrefixes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.sku {
            pulumi_sdk::resolve_input("sku", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            idle_timeout_in_minutes: registered.outputs.get("idleTimeoutInMinutes")
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
            public_ip_addresses: registered.outputs.get("publicIpAddresses")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_prefixes: registered.outputs.get("publicIpPrefixes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            subnets: registered.outputs.get("subnets")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            tags: registered.outputs.get("tags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            zones: registered.outputs.get("zones")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
