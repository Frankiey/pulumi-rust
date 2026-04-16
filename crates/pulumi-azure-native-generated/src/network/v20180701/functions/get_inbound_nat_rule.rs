use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified load balancer inbound nat rule.
#[derive(Default)]
pub struct GetInboundNatRuleArgs {
    /// Expands referenced resources.
    pub expand: Option<String>,
    /// The name of the inbound nat rule.
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
    /// A reference to a private IP address defined on a network interface of a VM. Traffic sent to the frontend port of each of the frontend IP configurations is forwarded to the backend IP.
    pub backend_ip_configuration: network::v20180701::NetworkInterfaceIPConfigurationResponse,
    /// The port used for the internal endpoint. Acceptable values range from 1 to 65535.
    pub backend_port: Option<i64>,
    /// Configures a virtual machine's endpoint for the floating IP capability required to configure a SQL AlwaysOn Availability Group. This setting is required when using the SQL AlwaysOn Availability Groups in SQL server. This setting can't be changed after you create the endpoint.
    pub enable_floating_ip: Option<bool>,
    /// Receive bidirectional TCP Reset on TCP flow idle timeout or unexpected connection termination. This element is only used when the protocol is set to TCP.
    pub enable_tcp_reset: Option<bool>,
    /// A unique read-only string that changes whenever the resource is updated.
    pub etag: Option<String>,
    /// A reference to frontend IP addresses.
    pub frontend_ip_configuration: Option<network::v20180701::SubResourceResponse>,
    /// The port for the external endpoint. Port numbers for each rule must be unique within the Load Balancer. Acceptable values range from 1 to 65534.
    pub frontend_port: Option<i64>,
    /// Resource ID.
    pub id: Option<String>,
    /// The timeout for the TCP idle connection. The value can be set between 4 and 30 minutes. The default value is 4 minutes. This element is only used when the protocol is set to TCP.
    pub idle_timeout_in_minutes: Option<i64>,
    /// Gets name of the resource that is unique within a resource group. This name can be used to access the resource.
    pub name: Option<String>,
    /// The transport protocol for the endpoint. Possible values are 'Udp' or 'Tcp' or 'All'.
    pub protocol: Option<String>,
    /// Gets the provisioning state of the public IP resource. Possible values are: 'Updating', 'Deleting', and 'Failed'.
    pub provisioning_state: Option<String>,
}

/// Gets the specified load balancer inbound nat rule.
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
    let result = ctx.invoke("azure-native:network/v20180701:getInboundNatRule", invoke_args, &opts).await?;

    Ok(GetInboundNatRuleResult {
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        backend_ip_configuration: serde_json::from_value(result.fields.get("backendIPConfiguration").cloned().unwrap_or_default())?
            ,
        backend_port: result.fields.get("backendPort").cloned().map(serde_json::from_value).transpose()?,
        enable_floating_ip: result.fields.get("enableFloatingIP").cloned().map(serde_json::from_value).transpose()?,
        enable_tcp_reset: result.fields.get("enableTcpReset").cloned().map(serde_json::from_value).transpose()?,
        etag: result.fields.get("etag").cloned().map(serde_json::from_value).transpose()?,
        frontend_ip_configuration: result.fields.get("frontendIPConfiguration").cloned().map(serde_json::from_value).transpose()?,
        frontend_port: result.fields.get("frontendPort").cloned().map(serde_json::from_value).transpose()?,
        id: result.fields.get("id").cloned().map(serde_json::from_value).transpose()?,
        idle_timeout_in_minutes: result.fields.get("idleTimeoutInMinutes").cloned().map(serde_json::from_value).transpose()?,
        name: result.fields.get("name").cloned().map(serde_json::from_value).transpose()?,
        protocol: result.fields.get("protocol").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: result.fields.get("provisioningState").cloned().map(serde_json::from_value).transpose()?,
    })
}
