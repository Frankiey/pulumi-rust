use pulumi_sdk_test::{test_with_mocks, Mocks, TestResult};

struct SimpleMocks;

impl Mocks for SimpleMocks {
    fn new_resource(
        &self,
        resource_type: &str,
        name: &str,
        inputs: &serde_json::Value,
    ) -> Result<(String, serde_json::Value), String> {
        let id = format!("{name}-id");
        // Echo inputs back as outputs, plus add the resource type
        let mut outputs = inputs.clone();
        if let serde_json::Value::Object(ref mut map) = outputs {
            map.insert(
                "resourceType".into(),
                serde_json::Value::String(resource_type.into()),
            );
        }
        Ok((id, outputs))
    }
}

#[tokio::test]
async fn test_mock_basic() {
    let result: TestResult =
        test_with_mocks(SimpleMocks, "test-project", "dev", |ctx| async move {
            ctx.export(
                "hello",
                pulumi_sdk::Output::known(serde_json::json!("world")),
            )
            .await;
            Ok(())
        })
        .await
        .expect("test_with_mocks should succeed");

    // Stack itself is not recorded as a resource
    assert!(
        result.resources.is_empty(),
        "no custom resources registered"
    );
    assert_eq!(
        result.exports.get("hello"),
        Some(&serde_json::json!("world"))
    );
}
