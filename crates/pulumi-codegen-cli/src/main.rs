//! CLI tool for generating Rust provider crates from Pulumi schemas.

use std::fs;
use std::path::{Path, PathBuf};
use std::process;

use clap::{Parser, Subcommand};
use pulumi_codegen::format::format_code;
use pulumi_codegen::modules::generate_module_tree;
use pulumi_codegen::schema::PulumiSchema;

#[derive(Parser)]
#[command(name = "pulumi-codegen-rust")]
#[command(about = "Generate Rust provider crates from Pulumi schemas")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a Rust crate from a Pulumi schema JSON file.
    Generate {
        /// Path to the Pulumi schema JSON file.
        #[arg(long)]
        schema: PathBuf,

        /// Output directory for the generated crate.
        #[arg(long)]
        out: PathBuf,

        /// Override the package name (default: pulumi-<schema_name>).
        #[arg(long)]
        package_name: Option<String>,

        /// Override the package version.
        #[arg(long)]
        version: Option<String>,

        /// Skip formatting generated code with prettyplease.
        #[arg(long, default_value = "false")]
        no_format: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            schema,
            out,
            package_name,
            version,
            no_format,
        } => {
            if let Err(e) = run_generate(&schema, &out, package_name, version, no_format) {
                eprintln!("Error: {e}");
                process::exit(1);
            }
        }
    }
}

fn run_generate(
    schema_path: &Path,
    out_dir: &Path,
    package_name: Option<String>,
    version_override: Option<String>,
    no_format: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read and parse schema
    eprintln!("Reading schema from {}", schema_path.display());
    let schema_json = fs::read_to_string(schema_path)?;
    let mut schema: PulumiSchema = serde_json::from_str(&schema_json)?;

    // Apply overrides
    if let Some(name) = &package_name {
        schema.name = name.clone();
    }
    if let Some(ver) = &version_override {
        schema.version = Some(ver.clone());
    }

    eprintln!(
        "Generating crate for {} v{}",
        schema.name,
        schema.version.as_deref().unwrap_or("0.1.0")
    );

    // Generate module tree
    let tree = generate_module_tree(&schema);

    // Write Cargo.toml
    let cargo_path = out_dir.join("Cargo.toml");
    fs::create_dir_all(out_dir)?;
    fs::write(&cargo_path, &tree.cargo_toml)?;
    eprintln!("  wrote {}", cargo_path.display());

    // Write all generated files
    let mut file_count = 0;
    for file in &tree.files {
        let file_path = out_dir.join(&file.path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = if no_format || !file.path.ends_with(".rs") {
            file.content.clone()
        } else {
            format_code(&file.content)
        };

        fs::write(&file_path, content)?;
        file_count += 1;
    }

    eprintln!(
        "  wrote {file_count} source files to {}",
        out_dir.display()
    );
    eprintln!("Done!");

    Ok(())
}
