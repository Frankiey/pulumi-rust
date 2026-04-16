//! Test utilities for Pulumi Rust SDK.
//!
//! Provides a mock monitor and engine that let you unit-test Pulumi programs
//! without connecting to a real Pulumi CLI or deploying any resources.

mod integration;
mod mock_monitor;

pub use integration::{DestroyResult, PulumiTestStack, StackOutput};
pub use mock_monitor::{test_with_mocks, MockResource, Mocks, TestResult};
