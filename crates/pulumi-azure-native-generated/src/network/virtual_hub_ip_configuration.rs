use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// IpConfigurations.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct VirtualHubIpConfigurationArgs {
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The name of the ipconfig.
    pub ip_config_name: Option<Input<String>>,
    /// Name of the Ip Configuration.
    pub name: Option<Input<String>>,
    /// The private IP address of the IP configuration.
    pub private_ip_address: Option<Input<String>>,
    /// The private IP address allocation method.
    pub private_ip_allocation_method: Option<Input<serde_json::Value>>,
    /// The reference to the public IP resource.
    pub public_ip_address: Option<Input<network::PublicIPAddressArgs>>,
    /// The resource group name of the VirtualHub.
    pub resource_group_name: Input<String>,
    /// The reference to the subnet resource.
    pub subnet: Option<Input<network::SubnetArgs>>,
    /// The name of the VirtualHub.
    pub virtual_hub_name: Input<String>,
}

/// IpConfigurations.
/// 
/// Uses Azure REST API version 2024-05-01. In version 2.x of the Azure Native provider, it used API version 2023-02-01.
/// 
/// Other available API versions: 2020-05-01, 2020-06-01, 2020-07-01, 2020-08-01, 2020-11-01, 2021-02-01, 2021-03-01, 2021-05-01, 2021-08-01, 2022-01-01, 2022-05-01, 2022-07-01, 2022-09-01, 2022-11-01, 2023-02-01, 2023-04-01, 2023-05-01, 2023-06-01, 2023-09-01, 2023-11-01, 2024-01-01, 2024-03-01, 2024-07-01, 2024-10-01, 2025-01-01, 2025-03-01, 2025-05-01. These can be accessed by generating a local SDK package using the CLI command `pulumi package add azure-native network [ApiVersion]`. See the [version guide](../../../version-guide/#accessing-any-api-version-via-local-packages) for details.
pub struct VirtualHubIpConfiguration {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// Name of the Ip Configuration.
    pub name: Output<serde_json::Value>,
    /// The private IP address of the IP configuration.
    pub private_ip_address: Output<serde_json::Value>,
    /// The private IP address allocation method.
    pub private_ip_allocation_method: Output<serde_json::Value>,
    /// The provisioning state of the IP configuration resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// The reference to the public IP resource.
    pub public_ip_address: Output<serde_json::Value>,
    /// The reference to the subnet resource.
    pub subnet: Output<serde_json::Value>,
    /// Ipconfiguration type.
    pub type_: Output<serde_json::Value>,
}

impl VirtualHubIpConfiguration {
    const TYPE_TOKEN: &'static str = "azure-native:network:VirtualHubIpConfiguration";

    /// Create a new VirtualHubIpConfiguration resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: VirtualHubIpConfigurationArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.ip_config_name {
            pulumi_sdk::resolve_input("ipConfigName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_ip_address {
            pulumi_sdk::resolve_input("privateIPAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.private_ip_allocation_method {
            pulumi_sdk::resolve_input("privateIPAllocationMethod", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.public_ip_address {
            pulumi_sdk::resolve_input("publicIPAddress", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("resourceGroupName", args.resource_group_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.subnet {
            pulumi_sdk::resolve_input("subnet", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("virtualHubName", args.virtual_hub_name, &mut inputs, &mut deps, &mut prop_deps).await;

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
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_ip_address: registered.outputs.get("privateIPAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            private_ip_allocation_method: registered.outputs.get("privateIPAllocationMethod")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            public_ip_address: registered.outputs.get("publicIPAddress")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            subnet: registered.outputs.get("subnet")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
