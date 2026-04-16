use std::collections::HashMap;

use pulumi_sdk::{Context, Input, Output, ResourceOptions, Result};

const RANDOM_STRING_TYPE: &str = "random:index/randomString:RandomString";

/// Input arguments for creating a RandomString resource.
pub struct RandomStringArgs {
    /// The length of the random string.
    pub length: Input<i64>,
    /// Include special characters. Defaults to true.
    pub special: Option<bool>,
    /// Include uppercase letters. Defaults to true.
    pub upper: Option<bool>,
    /// Include lowercase letters. Defaults to true.
    pub lower: Option<bool>,
    /// Include numeric characters. Defaults to true.
    pub numeric: Option<bool>,
}

/// A RandomString resource from the pulumi-random provider.
pub struct RandomString {
    pub urn: String,
    pub id: Output<serde_json::Value>,
    pub result: Output<serde_json::Value>,
}

impl RandomString {
    /// Create a new RandomString resource.
    pub async fn new(ctx: &Context, name: &str, args: RandomStringArgs) -> Result<Self> {
        let mut inputs = HashMap::new();
        let mut prop_deps: HashMap<String, Vec<String>> = HashMap::new();

        // Handle length input
        match args.length {
            Input::Value(v) => {
                inputs.insert("length".to_string(), serde_json::json!(v));
            }
            Input::Output(ref output) => {
                // Collect deps from the output
                prop_deps.insert("length".to_string(), output.deps().to_vec());

                // Wait for the value
                match output.wait().await {
                    pulumi_sdk::OutputState::Known(v) => {
                        inputs.insert("length".to_string(), serde_json::json!(v));
                    }
                    _ => {
                        // Unknown or pending — send null, engine handles preview
                        inputs.insert("length".to_string(), serde_json::Value::Null);
                    }
                }
            }
        }

        if let Some(special) = args.special {
            inputs.insert("special".to_string(), serde_json::json!(special));
        }
        if let Some(upper) = args.upper {
            inputs.insert("upper".to_string(), serde_json::json!(upper));
        }
        if let Some(lower) = args.lower {
            inputs.insert("lower".to_string(), serde_json::json!(lower));
        }
        if let Some(numeric) = args.numeric {
            inputs.insert("numeric".to_string(), serde_json::json!(numeric));
        }

        let registered = ctx
            .register_resource(RANDOM_STRING_TYPE, name, inputs, prop_deps, &ResourceOptions::default())
            .await?;

        let result = registered
            .outputs
            .get("result")
            .cloned()
            .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()]));

        let id = registered
            .outputs
            .get("id")
            .cloned()
            .unwrap_or_else(|| Output::unknown(vec![registered.urn.clone()]));

        Ok(Self {
            urn: registered.urn,
            id,
            result,
        })
    }
}
