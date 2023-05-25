#![allow(dead_code, unused_variables, unused_imports)]

pub mod client;
pub mod example;
pub mod models;
pub mod openapi_serialization;
pub mod security;
pub mod server;

pub use models::*;

pub const VERSION: &str = "{{version}}";
