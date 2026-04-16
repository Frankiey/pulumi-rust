//! Pulumi schema-driven code generation for Rust.
//!
//! This crate reads Pulumi provider schema JSON files and generates
//! idiomatic Rust crates with typed resources, functions, enums,
//! and module trees.

pub mod enums;
pub mod format;
pub mod functions;
pub mod modules;
pub mod naming;
pub mod provider;
pub mod resources;
pub mod schema;
pub mod types;
