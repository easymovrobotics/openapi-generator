use crate::OpenApiGenerator;
use anyhow::{Context, Result};
use schematools::{
    process::dereference::DereferencerOptions, schema::Schema, storage::SchemaStorage, Client,
};
use std::{env, fs, path::Path};
use url::Url;

pub fn generate(
    specification_file_path: impl AsRef<Path>,
    template_path: impl AsRef<Path>,
    package_name: impl AsRef<str>,
    version: impl AsRef<str>,
) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_file_path = specification_file_path.as_ref().with_extension("json");
    let out_file_name = out_file_path.file_name().context("bad filename")?;
    let schema_file_path = Path::new(&out_dir).join(out_file_name);
    let mut openapi_generator = OpenApiGenerator::new(
        &[schema_file_path],
        template_path,
        package_name.as_ref(),
        version.as_ref(),
    )?;
    openapi_generator
        .render(&out_dir)
        .context(format!("cannot render to `{out_dir:?}`"))?;
    Ok(())
}
