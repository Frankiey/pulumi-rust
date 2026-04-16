use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};
use std::collections::HashMap;

/// Input arguments for creating this resource.
///
/// Inbound NAT rule of the load balancer.
pub struct InboundNatRuleArgs {
    /// The port used for the internal endpoint. Acceptable values range from 1 to 65535.
    pub backend_port: Option<Input<i64>>,
    /// Configures a virtual machine's endpoint for the floating IP capability required to configure a SQL AlwaysOn Availability Group. This setting is required when using the SQL AlwaysOn Availability Groups in SQL server. This setting can't be changed after you create the endpoint.
    pub enable_floating_ip: Option<Input<bool>>,
    /// Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP.
    pub enable_tcp_reset: Option<Input<bool>>,
    /// A reference to frontend IP addresses.
    pub frontend_ip_configuration: Option<Input<network::v20200301::SubResourceArgs>>,
    /// The port for the external endpoint. Port numbers for each rule must be unique within the Load Balancer. Acceptable values range from 1 to 65534.
    pub frontend_port: Option<Input<i64>>,
    /// Resource ID.
    pub id: Option<Input<String>>,
    /// The timeout for the TCP idle connection. The value can be set between 4 and 30 minutes. The default value is 4 minutes. This element is only used when the protocol is set to TCP.
    pub idle_timeout_in_minutes: Option<Input<i64>>,
    /// The name of the inbound nat rule.
    pub inbound_nat_rule_name: Option<Input<String>>,
    /// The name of the load balancer.
    pub load_balancer_name: Input<String>,
    /// The name of the resource that is unique within the set of inbound NAT rules used by the load balancer. This name can be used to access the resource.
    pub name: Option<Input<String>>,
    /// The reference to the transport protocol used by the load balancing rule.
    pub protocol: Option<Input<serde_json::Value>>,
    /// The name of the resource group.
    pub resource_group_name: Input<String>,
}

/// Inbound NAT rule of the load balancer.
pub struct InboundNatRule {
    /// The URN of this resource.
    pub urn: String,
    /// The provider-assigned unique ID.
    pub id: Output<serde_json::Value>,
    /// The Azure API version of the resource.
    pub azure_api_version: Output<serde_json::Value>,
    /// A reference to a private IP address defined on a network interface of a VM. Traffic sent to the frontend port of each of the frontend IP configurations is forwarded to the backend IP.
    pub backend_ip_configuration: Output<serde_json::Value>,
    /// The port used for the internal endpoint. Acceptable values range from 1 to 65535.
    pub backend_port: Output<serde_json::Value>,
    /// Configures a virtual machine's endpoint for the floating IP capability required to configure a SQL AlwaysOn Availability Group. This setting is required when using the SQL AlwaysOn Availability Groups in SQL server. This setting can't be changed after you create the endpoint.
    pub enable_floating_ip: Output<serde_json::Value>,
    /// Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP.
    pub enable_tcp_reset: Output<serde_json::Value>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Output<serde_json::Value>,
    /// A reference to frontend IP addresses.
    pub frontend_ip_configuration: Output<serde_json::Value>,
    /// The port for the external endpoint. Port numbers for each rule must be unique within the Load Balancer. Acceptable values range from 1 to 65534.
    pub frontend_port: Output<serde_json::Value>,
    /// The timeout for the TCP idle connection. The value can be set between 4 and 30 minutes. The default value is 4 minutes. This element is only used when the protocol is set to TCP.
    pub idle_timeout_in_minutes: Output<serde_json::Value>,
    /// The name of the resource that is unique within the set of inbound NAT rules used by the load balancer. This name can be used to access the resource.
    pub name: Output<serde_json::Value>,
    /// The reference to the transport protocol used by the load balancing rule.
    pub protocol: Output<serde_json::Value>,
    /// The provisioning state of the inbound NAT rule resource.
    pub provisioning_state: Output<serde_json::Value>,
    /// Type of the resource.
    pub type_: Output<serde_json::Value>,
}

impl InboundNatRule {
    const TYPE_TOKEN: &'static str = "azure-native:network/v20200301:InboundNatRule";

    /// Create a new InboundNatRule resource.
    pub async fn new(
        ctx: &Context,
        name: &str,
        args: InboundNatRuleArgs,
        opts: Option<ResourceOptions>,
    ) -> Result<Self> {
        let opts = opts.unwrap_or_default();
        let mut inputs = HashMap::new();
        let mut deps: Vec<String> = Vec::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        if let Some(v) = args.backend_port {
            pulumi_sdk::resolve_input("backendPort", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_floating_ip {
            pulumi_sdk::resolve_input("enableFloatingIP", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.enable_tcp_reset {
            pulumi_sdk::resolve_input("enableTcpReset", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.frontend_ip_configuration {
            pulumi_sdk::resolve_input("frontendIPConfiguration", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.frontend_port {
            pulumi_sdk::resolve_input("frontendPort", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.id {
            pulumi_sdk::resolve_input("id", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.idle_timeout_in_minutes {
            pulumi_sdk::resolve_input("idleTimeoutInMinutes", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.inbound_nat_rule_name {
            pulumi_sdk::resolve_input("inboundNatRuleName", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        pulumi_sdk::resolve_input("loadBalancerName", args.load_balancer_name, &mut inputs, &mut deps, &mut prop_deps).await;
        if let Some(v) = args.name {
            pulumi_sdk::resolve_input("name", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
        if let Some(v) = args.protocol {
            pulumi_sdk::resolve_input("protocol", v, &mut inputs, &mut deps, &mut prop_deps).await;
        }
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
            backend_ip_configuration: registered.outputs.get("backendIPConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            backend_port: registered.outputs.get("backendPort")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_floating_ip: registered.outputs.get("enableFloatingIP")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            enable_tcp_reset: registered.outputs.get("enableTcpReset")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            etag: registered.outputs.get("etag")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            frontend_ip_configuration: registered.outputs.get("frontendIPConfiguration")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            frontend_port: registered.outputs.get("frontendPort")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            idle_timeout_in_minutes: registered.outputs.get("idleTimeoutInMinutes")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            name: registered.outputs.get("name")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            protocol: registered.outputs.get("protocol")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            provisioning_state: registered.outputs.get("provisioningState")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
            type_: registered.outputs.get("type")
                .cloned()
                .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()])),
        })
    }
}
