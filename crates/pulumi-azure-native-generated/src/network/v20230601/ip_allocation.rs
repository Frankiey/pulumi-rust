use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// IpAllocation resource.
pub struct IpAllocationArgs {
    /// IpAllocation tags.
    pub allocation_tags: Option<HashMap<String, Input<String>>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the IpAllocation.
    pub ip_allocation_name: Option<Input<String>>,
    /// The IPAM allocation ID.
    pub ipam_allocation_id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The address prefix for the IpAllocation.
    pub prefix: Option<Input<String>>,
    /// The address prefix length for the IpAllocation.
    pub prefix_length: Option<Input<i64>>,
    /// The address prefix Type for the IpAllocation.
    pub prefix_type: Option<Input<serde_json::Value>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
    /// The type for the IpAllocation.
    pub type_: Option<Input<serde_json::Value>>,
}

/// IpAllocation resource.
pub struct IpAllocation {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// IpAllocation tags.
    pub allocation_tags: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The IPAM allocation ID.
    pub ipam_allocation_id: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The address prefix for the IpAllocation.
    pub prefix: Output<serde_json::Value>,
    /// The address prefix length for the IpAllocation.
    pub prefix_length: Output<serde_json::Value>,
    /// The address prefix Type for the IpAllocation.
    pub prefix_type: Output<serde_json::Value>,
    /// The Subnet that using the prefix of this IpAllocation resource.
    pub subnet: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
    /// The VirtualNetwork that using the prefix of this IpAllocation resource.
    pub virtual_network: Output<serde_json::Value>,
}

impl IpAllocation {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20230601:IpAllocation";

    /// Create a new IpAllocation resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: IpAllocationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.allocation_tags {
            pulumi_sdk::resolve_input_map("allocationTags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_allocation_name {
            pulumi_sdk::resolve_input("ipAllocationName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ipam_allocation_id {
            pulumi_sdk::resolve_input("ipamAllocationId", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.prefix {
            pulumi_sdk::resolve_input("prefix", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.prefix_length {
            pulumi_sdk::resolve_input("prefixLength", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.prefix_type {
            pulumi_sdk::resolve_input("prefixType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.tags {
            pulumi_sdk::resolve_input_map("tags", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.type_ {
            pulumi_sdk::resolve_input("type", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            allocation_tags: registered.outputs.get("allocationTags")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            ipam_allocation_id: registered.outputs.get("ipamAllocationId")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            prefix: registered.outputs.get("prefix")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            prefix_length: registered.outputs.get("prefixLength")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            prefix_type: registered.outputs.get("prefixType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            subnet: registered.outputs.get("subnet")
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
        })
    }
}
