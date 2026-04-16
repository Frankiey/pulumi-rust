use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Service Endpoint policy definitions.
pub struct ServiceEndpointPolicyDefinitionArgs {
    /// A description for this rule. Restricted to 140 chars.
    pub description: Option<Input<String>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
    /// Service endpoint name.
    pub service: Option<Input<String>>,
    /// The name of the service endpoint policy definition name.
    pub service_endpoint_policy_definition_name: Option<Input<String>>,
    /// The name of the service endpoint policy.
    pub service_endpoint_policy_name: Input<String>,
    /// A list of service resources.
    pub service_resources: Option<Vec<Input<String>>>,
    /// The type of the resource.
    pub type_: Option<Input<String>>,
}

/// Service Endpoint policy definitions.
pub struct ServiceEndpointPolicyDefinition {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A description for this rule. Restricted to 140 chars.
    pub description: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// The name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The provisioning state of the service endpoint policy definition resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Service endpoint name.
    pub service: Output<serde_json::Value>,
    /// A list of service resources.
    pub service_resources: Output<serde_json::Value>,
    /// The type of the resource.
    pub type_: Output<serde_json::Value>,
}

impl ServiceEndpointPolicyDefinition {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20220501:ServiceEndpointPolicyDefinition";

    /// Create a new ServiceEndpointPolicyDefinition resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ServiceEndpointPolicyDefinitionArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.description {
            pulumi_sdk::resolve_input("description", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.service {
            pulumi_sdk::resolve_input("service", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.service_endpoint_policy_definition_name {
            pulumi_sdk::resolve_input("serviceEndpointPolicyDefinitionName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("serviceEndpointPolicyName", args.service_endpoint_policy_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.service_resources {
            pulumi_sdk::resolve_input_vec("serviceResources", v, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            description: registered.outputs.get("description")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service: registered.outputs.get("service")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            service_resources: registered.outputs.get("serviceResources")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
