use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified NSP association by name.
#[derive(Default)]
pub struct GetNetworkSecurityPerimeterAssociationArgs {
    /// The name of the NSP association.
    pub association_name: String,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNetworkSecurityPerimeterAssociationResult {
    /// Access mode on the association.
    pub access_mode: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Specifies if there are provisioning issues
    pub has_provisioning_issues: String,
    /// Fully qualified resource ID for the resource. E.g. "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/{resourceProviderNamespace}/{resourceType}/{resourceName}"
    pub id: String,
    /// The name of the resource
    pub name: String,
    /// The PaaS resource to be associated.
    pub private_link_resource: Option<network::v20241001::SubResourceResponse>,
    /// Profile id to which the PaaS resource is associated.
    pub profile: Option<network::v20241001::SubResourceResponse>,
    /// The provisioning state of the resource  association resource.
    pub provisioning_state: String,
    /// Azure Resource Manager metadata containing createdBy and modifiedBy information.
    pub system_data: network::v20241001::SystemDataResponse,
    /// The type of the resource. E.g. "Microsoft.Compute/virtualMachines" or "Microsoft.Storage/storageAccounts"
    pub type_: String,
}

/// Gets the specified NSP association by name.
pub async fn get_network_security_perimeter_association(
    ctx: &Context,
    args: GetNetworkSecurityPerimeterAssociationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNetworkSecurityPerimeterAssociationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("associationName".to_string(), json!(args.association_name));
    invoke_args.insert("networkSecurityPerimeterName".to_string(), json!(args.network_security_perimeter_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network/v20241001:getNetworkSecurityPerimeterAssociation", invoke_args, &opts).await?;

    Ok(GetNetworkSecurityPerimeterAssociationResult {
        access_mode: result.fields.get("accessMode").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        has_provisioning_issues: serde_json::from_value(result.fields.get("hasProvisioningIssues").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        private_link_resource: result.fields.get("privateLinkResource").cloned().map(serde_json::from_value).transpose()?,
        profile: result.fields.get("profile").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        system_data: serde_json::from_value(result.fields.get("systemData").cloned().unwrap_or_default())?
            ,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
