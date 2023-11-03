use crate::OpenApiGenerator;
use anyhow::{Context, Result};
use std::process::Command;
use std::{env, path::Path};

pub fn generate(
    specification_file_path: &[impl AsRef<Path>],
    template_path: impl AsRef<Path>,
) -> Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let mut openapi_generator = OpenApiGenerator::new(specification_file_path, template_path)?;
    openapi_generator
        .render(&out_dir)
        .context(format!("cannot render to `{out_dir:?}`"))?;
    if let Ok(rustfmt) = which::which("rustfmt") {
        for entry in glob::glob("**/*.rs")? {
            let Ok(entry) = entry else { continue };
            eprintln!("formatting {entry:?}");
            let mut child = Command::new(&rustfmt)
                .args(&[entry])
                .spawn()
                .context("cannot run rustfmt")?;
            child.wait().context("rustfmt wasn't running")?;
        }
    }
    Ok(())
}
