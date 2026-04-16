use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified load balancer inbound NAT rule.
#[derive(Default)]
pub struct GetInboundNatRuleArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the inbound NAT rule.
    pub inbound_nat_rule_name: String,
    /// The name of the load balancer.
    pub load_balancer_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetInboundNatRuleResult {
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// A reference to backendAddressPool resource.
    pub backend_address_pool: Option<network::v20220701::SubResourceResponse>,
    /// A reference to a private IP address defined on a network interface of a VM. Traffic sent to the frontend port of each of the frontend IP configurations is forwarded to the backend IP.
    pub backend_ip_configuration: network::v20220701::NetworkInterfaceIPConfigurationResponse,
    /// The port used for the internal endpoint. Acceptable values range from 1 to 65535.
    pub backend_port: Option<i64>,
    /// Configures a virtual machine's endpoint for the floating IP capability required to configure a SQL AlwaysOn Availability Group. This setting is required when using the SQL AlwaysOn Availability Groups in SQL server. This setting can't be changed after you create the endpoint.
    pub enable_floating_ip: Option<bool>,
    /// Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP.
    pub enable_tcp_reset: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: String,
    /// A reference to frontend IP addresses.
    pub frontend_ip_configuration: Option<network::v20220701::SubResourceResponse>,
    /// The port for the external endpoint. Port numbers for each rule must be unique within the Load Balancer. Acceptable values range from 1 to 65534.
    pub frontend_port: Option<i64>,
    /// The port range end for the external endpoint. This property is used together with BackendAddressPool and FrontendPortRangeStart. Individual inbound NAT rule port mappings will be created for each backend address from BackendAddressPool. Acceptable values range from 1 to 65534.
    pub frontend_port_range_end: Option<i64>,
    /// The port range start for the external endpoint. This property is used together with BackendAddressPool and FrontendPortRangeEnd. Individual inbound NAT rule port mappings will be created for each backend address from BackendAddressPool. Acceptable values range from 1 to 65534.
    pub frontend_port_range_start: Option<i64>,
    /// Resource ID.
    pub id: Option<String>,
    /// The timeout for the TCP idle connection. The value can be set between 4 and 30 minutes. The default value is 4 minutes. This element is only used when the protocol is set to TCP.
    pub idle_timeout_in_minutes: Option<i64>,
    /// The name of the resource that is unique within the set of inbound NAT rules used by the load balancer. This name can be used to access the resource.
    pub name: Option<String>,
    /// The reference to the transport protocol used by the load balancing rule.
    pub protocol: Option<String>,
    /// The provisioning state of the inbound NAT rule resource.
    pub provisioning_state: String,
    /// Type of the resource.
    pub type_: String,
}

/// Gets the specified load balancer inbound NAT rule.
pub async fn get_inbound_nat_rule(
    ctx: &Context,
    args: GetInboundNatRuleArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetInboundNatRuleResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.expand {
        invoke_args.insert("expand".to_string(), json!(v));
    }
    invoke_args.insert("inboundNatRuleName".to_string(), json!(args.inbound_nat_rule_name));
    invoke_args.insert("loadBalancerName".to_string(), json!(args.load_balancer_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20220701:getInboundNatRule", invoke_args, &opts).await?;

    Ok(GetInboundNatRuleResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        backend_address_pool: result.fields.get("backendAddressPool").cloned().map(serde_json::from_value).transpose()?,
        backend_ip_configuration: serde_json::from_value(result.fields.get("backendIPConfiguration").cloned().unwrap_or_default())?
            ,
        backend_port: result.fields.get("backendPort").cloned().map(serde_json::from_value).transpose()?,
        enable_floating_ip: result.fields.get("enableFloatingIP").cloned().map(serde_json::from_value).transpose()?,
        enable_tcp_reset: result.fields.get("enableTcpReset").cloned().map(serde_json::from_value).transpose()?,
        etag: serde_json::from_value(result.fields.get("etag").cloned().unwrap_or_default())?
            ,
        frontend_ip_configuration: result.fields.get("frontendIPConfiguration").cloned().map(serde_json::from_value).transpose()?,
        frontend_port: result.fields.get("frontendPort").cloned().map(serde_json::from_value).transpose()?,
        frontend_port_range_end: result.fields.get("frontendPortRangeEnd").cloned().map(serde_json::from_value).transpose()?,
        frontend_port_range_start: result.fields.get("frontendPortRangeStart").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        idle_timeout_in_minutes: result.fields.get("idleTimeoutInMinutes").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        protocol: result.fields.get("protocol").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
