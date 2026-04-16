use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// The network group resource
pub struct NetworkGroupArgs {
    /// Network group conditional filter.
    pub conditional_membership: Option<Input<String>>,
    /// A description of the network group.
    pub description: Option<Input<String>>,
    /// A friendly name for the network group.
    pub display_name: Option<Input<String>>,
    /// Group members of network group.
    pub group_members: Option<Vec<Input<network::v20210201preview::GroupMembersItemArgs>>>,
    /// Group member type.
    pub member_type: Option<Input<String>>,
    /// The name of the network group to get.
    pub network_group_name: Option<Input<String>>,
    /// The name of the network manager.
    pub network_manager_name: Input<String>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// The network group resource
pub struct NetworkGroup {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Network group conditional filter.
    pub conditional_membership: Output<serde_json::Value>,
    /// A description of the network group.
    pub description: Output<serde_json::Value>,
    /// A friendly name for the network group.
    pub display_name: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Group members of network group.
    pub group_members: Output<serde_json::Value>,
    /// Group member type.
    pub member_type: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the scope assignment resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The system metadata related to this resource.
    pub system_data: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl NetworkGroup {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20210201preview:NetworkGroup";

    /// Create a new NetworkGroup resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: NetworkGroupArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.conditional_membership {
            pulumi_sdk::resolve_input("conditionalMembership", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.display_name {
            pulumi_sdk::resolve_input("displayName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.group_members {
            pulumi_sdk::resolve_input_vec("groupMembers", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.member_type {
            pulumi_sdk::resolve_input("memberType", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.network_group_name {
            pulumi_sdk::resolve_input("networkGroupName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("networkManagerName", args.network_manager_name, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            conditional_membership: registered.outputs.get("conditionalMembership")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            display_name: registered.outputs.get("displayName")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            group_members: registered.outputs.get("groupMembers")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            member_type: registered.outputs.get("memberType")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            system_data: registered.outputs.get("systemData")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
