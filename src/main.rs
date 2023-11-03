use anyhow::{Context, Result};
use openapi_generator::OpenApiGenerator;
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
}

fn main() -> Result<()> {
    pretty_env_logger::init();
    let args = Cli::from_args();
    let template = args.template;
    let specification_list = args
        .openapi
        .iter()
        .map(|openapi| openapi.to_string_lossy())
        .collect::<Vec<_>>()
        .join(", ");
    let mut openapi_generator =
        OpenApiGenerator::new(&args.openapi, &template).context(format!(
        "cannot create OpenAPI generator with specifications `{specification_list}` and template at `{template:?}`"
    ))?;
    let destination = &args.destination;
    openapi_generator
        .render(destination)
        .context(format!("cannot render to `{destination:?}`"))?;
    Ok(())
}
