mod helpers;
mod openapi_generator;

use crate::openapi_generator::OpenApiGenerator;
use anyhow::{Context, Result};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "openapi_generator",
    about = "Generate code from OpenAPI specifications"
)]
struct Cli {
    /// Path of the template to generate
    template: PathBuf,
    /// Path of the OpenAPI specification files to use for generation
    openapi: Vec<PathBuf>,
    #[structopt(short = "d", long = "dest", default_value = "output")]
    /// Destination of the generated code
    destination: PathBuf,
    #[structopt(short = "p", long, default_value = "api")]
    package_name: String,
    #[structopt(short = "v", long, default_value = "0.0.1")]
    version: String,
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Cli::from_args();
    let mut openapi_generator = OpenApiGenerator::new(
        &args.openapi,
        &args.template,
        &args.package_name,
        &args.version,
    )
    .context(format!(
        "Cannot create OpenAPI generator with specifications `{}` and template at `{}`",
        args.openapi
            .iter()
            .map(|openapi| openapi.to_string_lossy())
            .collect::<Vec<_>>()
            .join(", "),
        args.template.to_string_lossy()
    ))?;
    openapi_generator
        .render(args.destination.clone())
        .context(format!(
            "Cannot render to `{}`",
            args.destination.to_string_lossy()
        ))?;
    Ok(())
}
