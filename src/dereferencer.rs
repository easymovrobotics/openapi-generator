use anyhow::Context;
use openapiv3::{
    Components, OpenAPI, Operation, ReferenceOr, RequestBody, Response, Schema, SchemaKind, Type,
};
use std::{borrow::BorrowMut, collections::HashMap, fs::File, path::Path};
use url::Url;

#[derive(Default, Debug)]
pub struct Dereferencer {
    pub known_refs: HashMap<String, Schema>,
}

impl Dereferencer {
    pub fn deref_specs<'a>(&mut self, specs: impl Iterator<Item = &'a mut OpenAPI>) {
        for spec in specs {
            self.deref_spec(spec);
        }
    }

    pub fn deref_spec(&mut self, spec: &mut OpenAPI) {
        let Some(file_path) = spec.extensions.get("file_path").and_then(|path| path.as_str()) else { return };
        let Ok(base_url) = Url::from_file_path(Path::new(file_path)) else { return };
        for paths in spec.paths.paths.values_mut() {
            let ReferenceOr::Item(path_item) = paths else { continue };
            for parameter in &path_item.parameters {
                if let Some(operation) = &mut path_item.get {
                    operation.parameters.push(parameter.clone())
                }
                if let Some(operation) = &mut path_item.put {
                    operation.parameters.push(parameter.clone())
                }
                if let Some(operation) = &mut path_item.post {
                    operation.parameters.push(parameter.clone())
                }
                if let Some(operation) = &mut path_item.delete {
                    operation.parameters.push(parameter.clone())
                }
                if let Some(operation) = &mut path_item.options {
                    operation.parameters.push(parameter.clone())
                }
                if let Some(operation) = &mut path_item.head {
                    operation.parameters.push(parameter.clone())
                }
                if let Some(operation) = &mut path_item.patch {
                    operation.parameters.push(parameter.clone())
                }
                if let Some(operation) = &mut path_item.trace {
                    operation.parameters.push(parameter.clone())
                }
            }
            self.deref_operation(&base_url, &mut path_item.get);
            self.deref_operation(&base_url, &mut path_item.put);
            self.deref_operation(&base_url, &mut path_item.post);
            self.deref_operation(&base_url, &mut path_item.delete);
            self.deref_operation(&base_url, &mut path_item.options);
            self.deref_operation(&base_url, &mut path_item.head);
            self.deref_operation(&base_url, &mut path_item.patch);
            self.deref_operation(&base_url, &mut path_item.trace);
        }
        for (ref_name, schema) in &self.known_refs {
            if spec.components.is_none() {
                spec.components = Some(Components::default());
            }
            let Some(components) = spec.components.as_mut() else { continue };
            components
                .schemas
                .insert(ref_name.clone(), ReferenceOr::Item(schema.clone()));
        }
    }

    pub fn deref_operation(&mut self, base_url: &Url, operation: &mut Option<Operation>) {
        let Some(operation) = operation else { return };
        self.deref_request_body(base_url, &mut operation.request_body);
        for response in operation.responses.responses.values_mut() {
            self.deref_response(base_url, response);
        }
    }

    pub fn deref_request_body(
        &mut self,
        base_url: &Url,
        request_body: &mut Option<ReferenceOr<RequestBody>>,
    ) {
        let Some(ReferenceOr::Item(request_body)) = request_body else { return };
        for media_type in request_body.content.values_mut() {
            let Some(schema) = &mut media_type.schema else { continue };
            self.deref_schema(base_url, schema);
        }
    }

    pub fn deref_response(&mut self, base_url: &Url, response: &mut ReferenceOr<Response>) {
        let ReferenceOr::Item(response) = response else { return };
        for media_type in response.content.values_mut() {
            let Some(schema) = &mut media_type.schema else { continue };
            self.deref_schema(base_url, schema);
        }
    }

    pub fn deref_schema(&mut self, base_url: &Url, schema: &mut ReferenceOr<Schema>) {
        match schema {
            ReferenceOr::Item(schema) => {
                self.deref_nested_schemas(base_url, schema);
            }
            ReferenceOr::Reference { reference } => {
                if let Some(reference) = self.deref_schema_reference(base_url, reference) {
                    *schema = ReferenceOr::Reference { reference };
                }
            }
        }
    }

    pub fn deref_schema_box(&mut self, base_url: &Url, items: &mut ReferenceOr<Box<Schema>>) {
        match items {
            ReferenceOr::Item(schema) => {
                self.deref_nested_schemas(base_url, schema);
            }
            ReferenceOr::Reference { reference } => {
                if let Some(reference) = self.deref_schema_reference(base_url, reference) {
                    *items = ReferenceOr::Reference { reference };
                }
            }
        }
    }

    pub fn deref_nested_schemas(&mut self, base_url: &Url, schema: &mut Schema) {
        match &mut schema.schema_kind {
            SchemaKind::Type(schema_type) => match schema_type {
                Type::Object(object) => {
                    for property in object.properties.values_mut() {
                        self.deref_schema_box(base_url, property);
                    }
                    if let Some(openapiv3::AdditionalProperties::Schema(additional_properties)) =
                        &mut object.additional_properties
                    {
                        self.deref_schema(base_url, additional_properties.borrow_mut());
                    }
                }
                Type::Array(array) => {
                    let Some(items) = &mut array.items else { return };
                    self.deref_schema_box(base_url, items);
                }
                _ => (),
            },
            SchemaKind::OneOf { .. } => todo!(),
            SchemaKind::AllOf { .. } => todo!(),
            SchemaKind::AnyOf { .. } => todo!(),
            SchemaKind::Not { .. } => todo!(),
            SchemaKind::Any(_) => todo!(),
        }
    }

    pub fn deref_schema_reference(&mut self, base_url: &Url, reference: &str) -> Option<String> {
        let Ok(url) = base_url.clone().join(reference) else { return None };
        if url.path() == base_url.path() {
            return None;
        }
        let Ok(file_path) = url.to_file_path() else { return None; };
        let Some(file_name) = file_path.file_stem() else { return None; };
        let file = File::open(&file_path)
            .context(format!("cannot open file: `{}`", file_path.display()))
            .unwrap();
        let mut schema = serde_yaml::from_reader(file).unwrap();
        self.deref_nested_schemas(&url, &mut schema);
        let name = file_name.to_string_lossy().into_owned();
        self.known_refs.insert(name.clone(), schema);
        Some(format!("#/components/schemas/{name}"))
    }
}

pub fn deref_specs<'a>(specs: impl Iterator<Item = &'a mut OpenAPI>) {
    let mut dereferencer = Dereferencer::default();
    dereferencer.deref_specs(specs);
}
