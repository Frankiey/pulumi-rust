fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_root = "../../proto";

    let protos = &[
        "pulumi/resource.proto",
        "pulumi/engine.proto",
        "pulumi/language.proto",
        "pulumi/provider.proto",
        "pulumi/plugin.proto",
        "pulumi/alias.proto",
        "pulumi/source.proto",
        "pulumi/callback.proto",
        "pulumi/codegen/hcl.proto",
    ];

    let proto_paths: Vec<String> = protos.iter().map(|p| format!("{proto_root}/{p}")).collect();

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_protos(&proto_paths, &[proto_root])?;

    for proto in protos {
        println!("cargo:rerun-if-changed={proto_root}/{proto}");
    }

    Ok(())
}
