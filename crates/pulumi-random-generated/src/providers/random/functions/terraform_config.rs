use pulumi_sdk::{Context, InvokeOptions, Result};
use serde_json::json;
use std::collections::HashMap;
/// Input arguments for this function.
///
/// This function returns a Terraform config object with terraform-namecased keys,to be used with the Terraform Module Provider.
#[derive(Default)]
pub struct TerraformConfigArgs {
    pub __self__: serde_json::Value,
}
/// Result of the function invocation.
pub struct TerraformConfigResult {
    pub result: HashMap<String, serde_json::Value>,
}
/// This function returns a Terraform config object with terraform-namecased keys,to be used with the Terraform Module Provider.
pub async fn terraform_config(
    ctx: &Context,
    args: TerraformConfigArgs,
    opts: Option<InvokeOptions>,
) -> Result<TerraformConfigResult> {
    let mut invoke_args = HashMap::new();
    invoke_args.insert("__self__".to_string(), json!(args.__self__));
    let opts = opts.unwrap_or_default();
    let result = ctx
        .invoke("pulumi:providers:random/terraformConfig", invoke_args, &opts)
        .await?;
    Ok(TerraformConfigResult {
        result: serde_json::from_value(
            result.fields.get("result").cloned().unwrap_or_default(),
        )?,
    })
}
