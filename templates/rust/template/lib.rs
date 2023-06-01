#![allow(dead_code, unused_variables, unused_imports)]

#[cfg(any(
{{~#each paths as |_ specName|}}
    feature = "{{snakecase specName "client"}}",
{{~/each}}
))]
pub mod client;
#[cfg(feature = "example")]
pub mod example;
pub mod models;
pub mod openapi_serialization;
#[cfg(any(
    {{~#each paths as |_ specName|}}
        feature = "{{snakecase specName "server"}}",
    {{~/each}}
    ))]
pub mod security;
#[cfg(any(
    {{~#each paths as |_ specName|}}
        feature = "{{snakecase specName "server"}}",
    {{~/each}}
    ))]
pub mod server;

pub use models::*;

pub const VERSION: &str = "{{version}}";
