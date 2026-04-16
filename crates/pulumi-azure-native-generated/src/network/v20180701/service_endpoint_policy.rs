use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Service End point policy resource.
pub struct ServiceEndpointPolicyArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// Resource location.
    pub location: Option<Input<String>>,
    /// The provisioning state of the service endpoint policy. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// The resource GUID property of the service endpoint policy resource.
    pub resource_guid: Option<Input<String>>,
    /// A collection of service endpoint policy definitions of the service endpoint policy.
    pub service_endpoint_policy_definitions: Option<Vec<Input<network::v20180701::ServiceEndpointPolicyDefinitionArgs>>>,
    /// The name of the service endpoint policy.
    pub service_endpoint_policy_name: Option<Input<String>>,
    /// Resource tags.
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Service End point policy resource.
pub struct ServiceEndpointPolicy {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Resource location.
    pub location: Output<serde_json::Value>,
    /// Resource name.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the service endpoint policy. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Output<serde_json::Value>,
    /// The resource GUID property of the service endpoint policy resource.
    pub resource_guid: Output<serde_json::Value>,
    /// A collection of service endpoint policy definitions of the service endpoint policy.
    pub service_endpoint_policy_definitions: Output<serde_json::Value>,
    /// Resource tags.
    pub tags: Output<serde_json::Value>,
    /// Resource type.
    pub type_: Output<serde_json::Value>,
}

impl ServiceEndpointPolicy {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20180701:ServiceEndpointPolicy";

    /// Create a new ServiceEndpointPolicy resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ServiceEndpointPolicyArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.provisioning_state {
            pulumi_sdk::resolve_input("provisioningState", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.resource_guid {
            pulumi_sdk::resolve_input("resourceGuid", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_endpoint_policy_definitions {
            pulumi_sdk::resolve_input_vec("serviceEndpointPolicyDefinitions", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_endpoint_policy_name {
            pulumi_sdk::resolve_input("serviceEndpointPolicyName", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            service_endpoint_policy_definitions: registered.outputs.get("serviceEndpointPolicyDefinitions")
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
