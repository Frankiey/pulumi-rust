use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Public IP prefix resource.
pub struct PublicIPPrefixArgs {
    /// The customIpPrefix that this prefix is associated with.
    pub custom_ip_prefix: Option<Input<network::v20241001::SubResourceArgs>>,
    /// The extended location of the public ip address.
    pub extended_location: Option<Input<network::v20241001::ExtendedLocationArgs>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The list of tags associated with the public IP prefix.
    pub ip_tags: Option<Vec<Input<network::v20241001::IpTagArgs>>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// NatGateway of Public IP Prefix.
    pub nat_gateway: Option<Input<network::v20241001::NatGatewayArgs>>,
    /// The Length of the Public IP Prefix.
    pub prefix_length: Option<Input<i64>>,
    /// The public IP address version.
    pub public_ip_address_version: Option<Input<serde_json::Value>>,
    /// The name of the public IP prefix.
    pub public_ip_prefix_name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The public IP prefix SKU.
    pub sku: Option<Input<network::v20241001::PublicIPPrefixSkuArgs>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Option<Vec<Input<String>>>,
}

/// Public IP prefix resource.
pub struct PublicIPPrefix {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// The customIpPrefix that this prefix is associated with.
    pub custom_ip_prefix: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The extended location of the public ip address.
    pub extended_location: Output<serde_json::Value>,
    /// The allocated Prefix.
    pub ip_prefix: Output<serde_json::Value>,
    /// The list of tags associated with the public IP prefix.
    pub ip_tags: Output<serde_json::Value>,
    /// The reference to load balancer frontend IP configuration associated with the public IP prefix.
    pub load_balancer_frontend_ip_configuration: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// NatGateway of Public IP Prefix.
    pub nat_gateway: Output<serde_json::Value>,
    /// The Length of the Public IP Prefix.
    pub prefix_length: Output<serde_json::Value>,
    /// The provisioning state of the public IP prefix resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The public IP address version.
    pub public_ip_address_version: Output<serde_json::Value>,
    /// The list of all referenced PublicIPAddresses.
    pub public_ip_addresses: Output<serde_json::Value>,
    /// The resource GUID property of the public IP prefix resource.
    pub resource_guid: Output<serde_json::Value>,
    /// The public IP prefix SKU.
    pub sku: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// A list of availability zones denoting the IP allocated for the resource needs to come from.
    pub zones: Output<serde_json::Value>,
}

impl PublicIPPrefix {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20241001:PublicIPPrefix";

    /// Create a new PublicIPPrefix resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: PublicIPPrefixArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.custom_ip_prefix {
            pulumi_sdk::resolve_input("customIPPrefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_tags {
            pulumi_sdk::resolve_input_vec("ipTags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.nat_gateway {
            pulumi_sdk::resolve_input("natGateway", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.prefix_length {
            pulumi_sdk::resolve_input("prefixLength", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_address_version {
            pulumi_sdk::resolve_input("publicIPAddressVersion", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_prefix_name {
            pulumi_sdk::resolve_input("publicIpPrefixName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            custom_ip_prefix: registered.outputs.get("customIPPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_prefix: registered.outputs.get("ipPrefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ip_tags: registered.outputs.get("ipTags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            load_balancer_frontend_ip_configuration: registered.outputs.get("loadBalancerFrontendIpConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            nat_gateway: registered.outputs.get("natGateway")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            prefix_length: registered.outputs.get("prefixLength")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_address_version: registered.outputs.get("publicIPAddressVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_addresses: registered.outputs.get("publicIPAddresses")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            resource_guid: registered.outputs.get("resourceGuid")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            sku: registered.outputs.get("sku")
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
