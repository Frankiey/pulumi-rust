use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// This operation retrieves the resiliency information for an Express Route Gateway, including the gateway's current resiliency score and recommendations to further improve the score
#[derive(Default)]
pub struct GetVirtualNetworkGatewayResiliencyInformationArgs {
    /// Attempt to recalculate the Resiliency Information for the gateway
    pub attempt_refresh: Option<bool>,
    /// The name of the resource group.
    pub resource_group_name: String,
    /// The name of the virtual network gateway.
    pub virtual_network_gateway_name: String,
}

/// Result of the function invocation.
pub struct GetVirtualNetworkGatewayResiliencyInformationResult {
    /// List of Resiliency based Recommendation Components for the gateway
    pub components: Option<Vec<network::v20250501::ResiliencyRecommendationComponentsResponse>>,
    /// Timestamp denoting the last time when the resiliency score was computed for the gateway
    pub last_computed_time: Option<String>,
    /// Maximum increase expected in the score if all of the recommendations are applied for the gateway
    pub max_score_from_recommendations: Option<String>,
    /// Minimum increase expected in the score if the at least one of the recommendations is applied for the gateway
    pub min_score_from_recommendations: Option<String>,
    /// Timestamp denoting the next eligible time to re-compute the resiliency score for the gateway
    pub next_eligible_compute_time: Option<String>,
    /// Current Resiliency Score for the gateway
    pub overall_score: Option<String>,
    /// Update in the Resiliency Score for the gateway from the last computed score
    pub score_change: Option<String>,
}

/// This operation retrieves the resiliency information for an Express Route Gateway, including the gateway's current resiliency score and recommendations to further improve the score
pub async fn get_virtual_network_gateway_resiliency_information(
    ctx: &Context,
    args: GetVirtualNetworkGatewayResiliencyInformationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetVirtualNetworkGatewayResiliencyInformationResult> {
    let mut invoke_args = HashMap::new();
    if let Some(v) = args.attempt_refresh {
        invoke_args.insert("attemptRefresh".to_string(), json!(v));
    }
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));
    invoke_args.insert("virtualNetworkGatewayName".to_string(), json!(args.virtual_network_gateway_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20250501:getVirtualNetworkGatewayResiliencyInformation", invoke_args, &opts).await?;

    Ok(GetVirtualNetworkGatewayResiliencyInformationResult {
        components: result.fields.get("components").cloned().map(serde_json::from_value).transpose()?,
        last_computed_time: result.fields.get("lastComputedTime").cloned().map(serde_json::from_value).transpose()?,
        max_score_from_recommendations: result.fields.get("maxScoreFromRecommendations").cloned().map(serde_json::from_value).transpose()?,
        min_score_from_recommendations: result.fields.get("minScoreFromRecommendations").cloned().map(serde_json::from_value).transpose()?,
        next_eligible_compute_time: result.fields.get("nextEligibleComputeTime").cloned().map(serde_json::from_value).transpose()?,
        overall_score: result.fields.get("overallScore").cloned().map(serde_json::from_value).transpose()?,
        score_change: result.fields.get("scoreChange").cloned().map(serde_json::from_value).transpose()?,
    })
}
