use anyhow::Result;
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError,
};
use json_pointer::JsonPointer;
use serde_json::value::Value as Json;

macro_rules! case_helper {
    ($name:ident, $function:ident) => {
        pub(crate) fn $name(
            helper: &Helper,
            _: &Handlebars,
            _: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            use heck::*;
            let values = helper
                .params()
                .iter()
                .map(|v| v.value().render())
                .collect::<Vec<_>>();
            let rendered = values.as_slice().join(" ").$function();
            out.write(rendered.as_ref())?;
            Ok(())
        }
    };
}

case_helper!(mixedcase, to_lower_camel_case);
case_helper!(camelcase, to_upper_camel_case);
case_helper!(snakecase, to_snake_case);
case_helper!(shoutysnakecase, to_shouty_snake_case);
handlebars_helper!(component_path: |ref_path: str| parse_component_path(ref_path));
handlebars_helper!(component_name: |ref_path: str| parse_component_name(ref_path));
handlebars_helper!(sanitize: |word: str| apply_sanitize(word));
handlebars_helper!(json: |data: Json| apply_json(data));
handlebars_helper!(is_http_code_success: |http_status: str| http_status.starts_with('1') || http_status.starts_with('2') || http_status.starts_with('3'));

pub(crate) fn parse_component_path(ref_path: &str) -> String {
    use heck::ToUpperCamelCase;
    let mut path = Vec::new();
    let mut pointer = ref_path.parse::<JsonPointer<_, _>>().unwrap();
    while let Some(segment) = pointer.pop() {
        path.push(segment);
    }
    if let Some(name) = path.first_mut() {
        *name = name.to_upper_camel_case()
    }
    path.reverse();
    path.join("::")
}

pub(crate) fn parse_component_name(ref_path: &str) -> String {
    use heck::ToUpperCamelCase;
    ref_path
        .parse::<JsonPointer<_, _>>()
        .unwrap()
        .pop()
        .map(|name| name.to_upper_camel_case())
        .unwrap_or_default()
}

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "typeof", "unsized", "virtual", "yield", "async", "await", "try",
];

pub(crate) fn apply_sanitize(word: &str) -> String {
    if KEYWORDS.iter().any(|&keyword| word == keyword) {
        format!("r#{word}")
    } else {
        word.to_string()
    }
}

pub(crate) fn has(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let data = helper
        .param(0)
        .ok_or_else(|| RenderError::new("data not found"))?
        .value();
    let field = helper
        .param(1)
        .ok_or_else(|| RenderError::new("field not found"))?
        .value()
        .as_str()
        .ok_or_else(|| RenderError::new("field is not a string"))?;
    let value = helper.param(2);
    let result = match data {
        serde_json::Value::Array(data) => {
            if let Some(value) = value {
                let value_converted = value
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderError::new("value is not a string"))?;
                data.iter()
                    .any(|list_elem| list_elem[field] == value_converted)
            } else {
                data.iter().any(|list_elem| list_elem == field)
            }
        }
        serde_json::Value::Object(data) => {
            if let Some(value) = value {
                let field_value = data
                    .get(field)
                    .ok_or_else(|| RenderError::new("field does not exist"))?;
                let value_converted = value
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderError::new("value is not a string"))?;
                field_value == value_converted
            } else {
                data.get(field).is_some()
            }
        }
        _ => false,
    };
    out.write(if result { "true" } else { "" })?;
    Ok(())
}

pub(crate) fn apply_json(data: &Json) -> String {
    data.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_component_path() {
        assert_eq!(
            parse_component_path("#/components/schemas/Pet"),
            "components::schemas::Pet".to_string()
        )
    }
}
