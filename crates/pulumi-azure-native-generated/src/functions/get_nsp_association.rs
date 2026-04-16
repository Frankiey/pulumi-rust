use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;

/// Input arguments for this function.
///
/// Gets the specified NSP association by name.
/// 
/// Uses Azure REST API version 2023-08-01-preview.
#[derive(Default)]
pub struct GetNspAssociationArgs {
    /// The name of the NSP association.
    pub association_name: String,
    /// The name of the network security perimeter.
    pub network_security_perimeter_name: String,
    /// The name of the resource group.
    pub resource_group_name: String,
}

/// Result of the function invocation.
pub struct GetNspAssociationResult {
    /// Access mode on the association.
    pub access_mode: Option<String>,
    /// The Azure API version of the resource.
    pub azure_api_version: String,
    /// Specifies if there are provisioning issues
    pub has_provisioning_issues: String,
    /// Resource ID.
    pub id: String,
    /// Resource location.
    pub location: Option<String>,
    /// Resource name.
    pub name: String,
    /// The PaaS resource to be associated.
    pub private_link_resource: Option<network::SubResourceResponse>,
    /// Profile id to which the PaaS resource is associated.
    pub profile: Option<network::SubResourceResponse>,
    /// The provisioning state of the resource  association resource.
    pub provisioning_state: String,
    /// Resource tags.
    pub tags: Option<HashMap<String, String>>,
    /// Resource type.
    pub type_: String,
}

/// Gets the specified NSP association by name.
pub async fn get_nsp_association(
    ctx: &Context,
    args: GetNspAssociationArgs,
    opts: Option<InvokeOptions>,
) -> Result<GetNspAssociationResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("associationName".to_string(), json!(args.association_name));
    invoke_args.insert("networkSecurityPerimeterName".to_string(), json!(args.network_security_perimeter_name));
    invoke_args.insert("resourceGroupName".to_string(), json!(args.resource_group_name));

    let opts = opts.unwrap_or_default();
    let result = ctx.invoke("azure-native:network:getNspAssociation", invoke_args, &opts).await?;

    Ok(GetNspAssociationResult {
        access_mode: result.fields.get("accessMode").cloned().map(serde_json::from_value).transpose()?,
        azure_api_version: serde_json::from_value(result.fields.get("azureApiVersion").cloned().unwrap_or_default())?
            ,
        has_provisioning_issues: serde_json::from_value(result.fields.get("hasProvisioningIssues").cloned().unwrap_or_default())?
            ,
        id: serde_json::from_value(result.fields.get("id").cloned().unwrap_or_default())?
            ,
        location: result.fields.get("location").cloned().map(serde_json::from_value).transpose()?,
        name: serde_json::from_value(result.fields.get("name").cloned().unwrap_or_default())?
            ,
        private_link_resource: result.fields.get("privateLinkResource").cloned().map(serde_json::from_value).transpose()?,
        profile: result.fields.get("profile").cloned().map(serde_json::from_value).transpose()?,
        provisioning_state: serde_json::from_value(result.fields.get("provisioningState").cloned().unwrap_or_default())?
            ,
        tags: result.fields.get("tags").cloned().map(serde_json::from_value).transpose()?,
        type_: serde_json::from_value(result.fields.get("type").cloned().unwrap_or_default())?
            ,
    })
}
