use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Resource information.
pub struct ResourceArgs {
    /// The API version to use for the operation.
    pub api_version: Input<String>,
    /// Resource extended location.
    pub extended_location: Option<Input<resources::v20230701::ExtendedLocationArgs>>,
    /// The identity of the resource.
    pub identity: Option<Input<resources::v20230701::IdentityArgs>>,
    /// The kind of the resource.
    pub kind: Option<Input<String>>,
    /// Resource location
    pub location: Option<Input<String>>,
    /// ID of the resource that manages this resource.
    pub managed_by: Option<Input<String>>,
    /// The parent resource identity.
    pub parent_resource_path: Input<String>,
    /// The plan of the resource.
    pub plan: Option<Input<resources::v20230701::PlanArgs>>,
    /// The resource properties.
    pub properties: Option<Input<serde_json::Value>>,
    /// The name of the resource group for the resource. The name is case insensitive.
    pub resource_group_name: Input<String>,
    /// The name of the resource to create.
    pub resource_name: Option<Input<String>>,
    /// The namespace of the resource provider.
    pub resource_provider_namespace: Input<String>,
    /// The resource type of the resource to create.
    pub resource_type: Input<String>,
    /// The SKU of the resource.
    pub sku: Option<Input<resources::v20230701::SkuArgs>>,
    /// Resource tags
    pub tags: Option<HashMap<String, Input<String>>>,
}

/// Resource information.
pub struct Resource {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// Resource extended location.
    pub extended_location: Output<serde_json::Value>,
    /// The identity of the resource.
    pub identity: Output<serde_json::Value>,
    /// The kind of the resource.
    pub kind: Output<serde_json::Value>,
    /// Resource location
    pub location: Output<serde_json::Value>,
    /// ID of the resource that manages this resource.
    pub managed_by: Output<serde_json::Value>,
    /// Resource name
    pub name: Output<serde_json::Value>,
    /// The plan of the resource.
    pub plan: Output<serde_json::Value>,
    /// The resource properties.
    pub properties: Output<serde_json::Value>,
    /// The SKU of the resource.
    pub sku: Output<serde_json::Value>,
    /// Resource tags
    pub tags: Output<serde_json::Value>,
    /// Resource type
    pub type_: Output<serde_json::Value>,
}

impl Resource {
    const TYPE_TOKEN: &'static str = "azure-native:resources/v20230701:Resource";

    /// Create a new Resource resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: ResourceArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        pulumi_sdk::resolve_input("apiVersion", args.api_version, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.extended_location {
            pulumi_sdk::resolve_input("extendedLocation", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.identity {
            pulumi_sdk::resolve_input("identity", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.kind {
            pulumi_sdk::resolve_input("kind", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.location {
            pulumi_sdk::resolve_input("location", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.managed_by {
            pulumi_sdk::resolve_input("managedBy", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("parentResourcePath", args.parent_resource_path, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.plan {
            pulumi_sdk::resolve_input("plan", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.properties {
            pulumi_sdk::resolve_input("properties", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.resource_name {
            pulumi_sdk::resolve_input("resourceName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceProviderNamespace", args.resource_provider_namespace, &mut inputs, &mut deps, &mut prop_deps).await;
        pulumi_sdk::resolve_input("resourceType", args.resource_type, &mut inputs, &mut deps, &mut prop_deps).await;
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
            azure_api_version: registered.outputs.get("azureApiVersion")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            extended_location: registered.outputs.get("extendedLocation")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            identity: registered.outputs.get("identity")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            kind: registered.outputs.get("kind")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            location: registered.outputs.get("location")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            managed_by: registered.outputs.get("managedBy")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            plan: registered.outputs.get("plan")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            properties: registered.outputs.get("properties")
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
        })
    }
}
