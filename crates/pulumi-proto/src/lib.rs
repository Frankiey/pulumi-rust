pub mod pulumirpc {
    #![allow(
        clippy::doc_lazy_continuation,
        clippy::doc_overindented_list_items,
        clippy::needless_question_mark
    )]
    tonic::include_proto!("pulumirpc");

    pub mod codegen {
        tonic::include_proto!("pulumirpc.codegen");
    }
}

// Re-export key types for convenience
pub use pulumirpc::engine_client::EngineClient;
pub use pulumirpc::language_runtime_server::{LanguageRuntime, LanguageRuntimeServer};
pub use pulumirpc::resource_monitor_client::ResourceMonitorClient;
