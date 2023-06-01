use crate::helpers::{
    camelcase, component_path, has, is_http_code_success, json, mixedcase, sanitize,
    shoutysnakecase, snakecase,
};
use anyhow::{anyhow, Context, Result};
use handlebars::Handlebars;
use heck::SnakeCase;
use openapiv3::OpenAPI;
use std::{
    collections::HashMap,
    fs::File,
    path::{Path, PathBuf},
};

pub struct OpenApiGenerator {
    handlebars: Handlebars,
    specs: HashMap<String, OpenAPI>,
    template_path: PathBuf,
    package_name: String,
    version: String,
}

impl OpenApiGenerator {
    pub fn new<T: AsRef<Path>, U: AsRef<Path>>(
        specs_path: &[T],
        template_path: U,
        package_name: &str,
        version: &str,
    ) -> Result<Self> {
        let mut specs = HashMap::new();
        for spec_path in specs_path {
            let (title, spec) = Self::parse_specification(&spec_path.as_ref())?;
            specs.insert(title, spec);
        }

        let mut openapi_generator = Self {
            handlebars: Handlebars::new(),
            specs,
            template_path: template_path.as_ref().join("template"),
            package_name: package_name.to_string(),
            version: version.to_string(),
        };

        let partials_path = template_path.as_ref().join("partials");
        openapi_generator
            .register_partials(&partials_path)
            .context(format!(
                "Failed to register partials from `{}`",
                partials_path.display()
            ))?;
        openapi_generator.register_helpers();
        Ok(openapi_generator)
    }

    fn parse_specification(specs_path: &Path) -> Result<(String, OpenAPI)> {
        let specs_string = std::fs::read_to_string(&specs_path).context(format!(
            "Cannot read specification file `{}`",
            specs_path.display()
        ))?;
        let open_api: OpenAPI = serde_yaml::from_str(&specs_string).context(format!(
            "Cannot parse specification file `{}`",
            specs_path.display()
        ))?;
        let title = open_api.info.title.to_snake_case();
        Ok((title, open_api))
    }

    fn register_helpers(&mut self) {
        self.handlebars
            .register_helper("camelcase", Box::new(camelcase));
        self.handlebars
            .register_helper("snakecase", Box::new(snakecase));
        self.handlebars
            .register_helper("shoutysnakecase", Box::new(shoutysnakecase));
        self.handlebars
            .register_helper("mixedcase", Box::new(mixedcase));
        self.handlebars
            .register_helper("component_path", Box::new(component_path));
        self.handlebars
            .register_helper("sanitize", Box::new(sanitize));
        self.handlebars.register_helper("has", Box::new(has));
        self.handlebars.register_helper("json", Box::new(json));
        self.handlebars
            .register_helper("is_http_code_success", Box::new(is_http_code_success));
    }

    fn register_partials<T: AsRef<Path>>(&mut self, partials_dir: T) -> Result<()> {
        for entry in walkdir::WalkDir::new(partials_dir) {
            if let Ok(entry) = entry {
                if entry.file_type().is_file() {
                    let path = entry.path();
                    let template_name = path
                        .file_stem()
                        .ok_or_else(|| anyhow!("File name is empty"))?
                        .to_str()
                        .ok_or_else(|| anyhow!("File path is not unicode"))?;
                    self.handlebars
                        .register_template_file(template_name, path)
                        .context(format!("Cannot register partial `{}`", path.display()))?;
                    log::info!(
                        "new partial registered: {} ({})",
                        template_name,
                        path.display()
                    );
                }
            }
        }
        Ok(())
    }

    fn get_paths(&self) -> Result<serde_yaml::Value> {
        let mut paths = serde_yaml::Mapping::new();
        for (title, spec) in &self.specs {
            paths.insert(
                serde_yaml::Value::String(title.to_string()),
                serde_yaml::to_value(&spec.paths.paths)?,
            );
        }
        Ok(serde_yaml::Value::Mapping(paths))
    }

    fn get_schemas(&self) -> Result<serde_yaml::Value> {
        let mut schemas = serde_yaml::Mapping::new();
        for (_title, spec) in &self.specs {
            for (name, schema) in &spec
                .components
                .as_ref()
                .map(|components| components.schemas.clone())
                .unwrap_or_default()
            {
                schemas.insert(
                    serde_yaml::Value::String(name.to_string()),
                    serde_yaml::to_value(schema)?,
                );
            }
        }
        Ok(serde_yaml::Value::Mapping(schemas))
    }

    pub fn get_template_data(&self) -> Result<serde_yaml::Value> {
        let mut root = serde_yaml::Mapping::new();
        root.insert(
            serde_yaml::Value::String("paths".to_string()),
            self.get_paths()?,
        );
        let mut components = serde_yaml::Mapping::new();
        components.insert(
            serde_yaml::Value::String("schemas".to_string()),
            self.get_schemas()?,
        );
        root.insert(
            serde_yaml::Value::String("components".to_string()),
            serde_yaml::Value::Mapping(components),
        );
        root.insert(
            serde_yaml::Value::String("openapi_generator_version".to_string()),
            serde_yaml::Value::String(env!("CARGO_PKG_VERSION").to_string()),
        );
        root.insert(
            serde_yaml::Value::String("package_name".to_string()),
            serde_yaml::Value::String(self.package_name.to_string()),
        );
        root.insert(
            serde_yaml::Value::String("version".to_string()),
            serde_yaml::Value::String(self.version.to_string()),
        );
        let template_data = serde_yaml::Value::Mapping(root);
        log::info!("{}", serde_yaml::to_string(&template_data)?);
        Ok(template_data)
    }

    pub fn render<T: AsRef<Path>>(&mut self, output_path: T) -> Result<()> {
        self.render_from_path(output_path.as_ref(), &PathBuf::new())
    }

    fn render_from_path(&mut self, output_path: &Path, path: &Path) -> Result<()> {
        let template_path = self.template_path.join(path);
        for entry in std::fs::read_dir(&template_path).context(format!(
            "Cannot walk into template directory `{}`",
            template_path.display()
        ))? {
            if let Ok(entry) = entry {
                if entry.file_type()?.is_file() {
                    let template_key = &format!("{}", path.join(entry.file_name()).display());
                    self.handlebars
                        .register_template_file(template_key, entry.path())
                        .context(format!(
                            "Cannot register template `{}` ",
                            entry.path().display()
                        ))?;
                    log::info!(
                        "new template registered: {} ({})",
                        template_key,
                        entry.path().display()
                    );
                    let output_file_path = output_path.join(path).join(entry.file_name());
                    let mut output_file = File::create(&output_file_path)?;
                    self.handlebars
                        .render_to_write(template_key, &self.get_template_data()?, &mut output_file)
                        .context(format!(
                            "Failed to render template `{}` at `{}`",
                            template_key,
                            output_file_path.display()
                        ))?;
                    log::info!("render {} to {}", template_key, output_file_path.display());
                } else if entry.file_type()?.is_dir() {
                    let mut path = path.to_path_buf();
                    path.push(entry.file_name());
                    let new_output_path = output_path.join(&path);
                    std::fs::create_dir_all(&new_output_path).context(format!(
                        "Cannot create directory `{}`",
                        new_output_path.display()
                    ))?;
                    log::info!("create {}", new_output_path.display());
                    self.render_from_path(output_path, &path).context(format!(
                        "Failed to render templates under `{}`",
                        new_output_path.display()
                    ))?;
                }
            }
        }
        Ok(())
    }
}
