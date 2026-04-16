use pulumi_sdk_test::PulumiTestStack;

/// Integration test for the random-strings example.
/// Requires: `pulumi` CLI on PATH.
/// Run with: `cargo test -p pulumi-sdk-test --test integration_test -- --ignored --nocapture`
#[tokio::test]
#[ignore] // requires pulumi CLI + credentials
async fn test_random_strings_lifecycle() {
    let stack = PulumiTestStack::new("examples/random-strings", "integration-test")
        .await
        .expect("failed to init stack");

    // Preview should succeed
    let preview = stack.preview().await.expect("preview failed");
    assert!(
        !preview.stdout.is_empty() || !preview.stderr.is_empty(),
        "preview should produce output"
    );

    // Up should succeed and produce outputs
    let up = stack.up().await.expect("up failed");
    assert!(!up.outputs.is_empty(), "up should produce stack outputs");

    // Clean up
    stack.cleanup().await.expect("cleanup failed");
}
