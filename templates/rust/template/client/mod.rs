#![allow(clippy::ptr_arg, clippy::let_unit_value)]
/* Reqwest's errors are bad-mannered and recurse on their source when displayed.
 * This behavior doesn't interact well with thiserror which also recurse on error's cause
 * when displayed. To prevent this issue, this wrapper hides the error's source from thiserror.
 */
pub struct ReqwestError {
    err: reqwest::Error,
}

impl ReqwestError {
    pub fn new(err: reqwest::Error) -> Self {
        Self { err }
    }
}

impl std::error::Error for ReqwestError {}

impl From<reqwest::Error> for ReqwestError {
    fn from(err: reqwest::Error) -> Self {
        Self::new(err)
    }
}

impl std::fmt::Debug for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(status) = self.err.status() {
            write!(f, "{:?}: {:?}", self.err, status)
        } else {
            std::fmt::Debug::fmt(&self.err, f)
        }
    }
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(status) = self.err.status() {
            write!(f, "{}: {}", self.err, status)
        } else {
            std::fmt::Display::fmt(&self.err, f)
        }
    }
}

{{~#each paths as |specPaths specName|}}
    #[cfg(feature = "{{snakecase @key "client"}}")]
    pub mod {{specName}} {
        use crate::client::ReqwestError;
        use std::time::Duration;
        use url::{Url};
        use std::{sync::Arc};

        #[derive(Clone)]
        pub struct {{camelcase specName "Client"}} {
            pub url: Url,
            pub client: reqwest::Client,
        }

        {{~#*inline "async_operation_fn"}}

            #[allow(clippy::unit_arg)]
            pub async fn {{snakecase operationId}}(
                &self,
                {{~#if ../parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
                {{~#if requestBody~}}
                    {{~#with requestBody.content.[application/json]}}body: &{{snakecase ../operationId}}::Body,{{~/with}}
                    {{~#with requestBody.content.[multipart/form-data]}}form: reqwest::multipart::Form,{{~/with}}
                {{/if~}}
            ) -> Result<{{snakecase operationId}}::Success, {{snakecase operationId}}::Error> {
                use {{snakecase ../operationId}}::*;
                let url = self.url.join(
                    {{#if (has parameters "in" "path")~}}
                    format!("{{@../key}}"
                    {{~#each parameters}}
                        {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
                    {{~/each~}})
                    {{~else~}}
                    "{{@../key}}"
                    {{~/if~}}
                    .trim_start_matches('/')
                ).expect("url parse error");
                let response = self.client
                    .{{operation_verb}}(url)
                    {{#if (has parameters "in" "query")~}}
                    .query(&parameters.query())
                    {{~/if}}
                    {{~#if requestBody}}
                        {{~#with requestBody.content.[application/json]}}.json(&body){{~/with}}
                        {{~#with requestBody.content.[multipart/form-data]}}.multipart(form){{~/with}}
                    {{~/if}}
                    .send().await.map_err(ReqwestError::new)?;
                match response.status().as_str() {
                    {{~#each responses}}
                    {{~#if (not (eq @key "default"))}}
                        {{~#if (eq @key "204")}}
                        "{{@key}}" => {
                            Ok(Success::{{camelcase "Status" @key}}(()))
                        }
                        {{~else~}}
                        "{{@key}}" => {
                            {{~#if content}}
                                {{~#with content.[image/png]}}let response_body = response.bytes().await.map_err(ReqwestError::new)?.into_iter().collect();{{~/with}}
                                {{~#with content.[image/jpeg]}}let response_body = response.bytes().await.map_err(ReqwestError::new)?.into_iter().collect();{{~/with}}
                                {{~#with content.[application/bag]}}let response_body = response.bytes().await.map_err(ReqwestError::new)?.into_iter().collect();{{~/with}}
                                {{~#with content.[application/pbstream]}}let response_body = response.bytes().await.map_err(ReqwestError::new)?.into_iter().collect();{{~/with}}
                                {{~#with content.[image/x-portable-graymap]}}let response_body = response.bytes().await.map_err(ReqwestError::new)?.into_iter().collect();{{~/with}}
                                {{~#with content.[text/plain]}}let response_body = response.text().await.map_err(ReqwestError::new)?;{{~/with}}
                                {{~#with content.[application/json]}}let response_body = response.json().await.map_err(ReqwestError::new)?;{{~/with}}
                            {{~else~}}
                                let response_body = ();
                            {{~/if}}
                            {{~#if (is_http_code_success @key)}}
                            Ok(Success::{{camelcase "Status" @key}}(response_body))
                            {{else}}
                            Err(Error::{{camelcase "Status" @key}}(response_body))
                            {{~/if}}
                        }
                        {{~/if}}
                    {{~/if}}
                    {{~/each}}
                        _ => Err(Error::unknown(response).await),
                }
            }
        {{~/inline}}

        impl {{camelcase specName "Client"}} {
            pub fn new(url: &str) -> Self {
                let url = Url::parse(url).expect("cannot parse url");
                Self {
                    url,
                    client: reqwest::Client::new(),
                }
            }

            pub fn with_timeout(mut self, timeout: Duration) -> Self {
                self.client = reqwest::Client::builder().timeout(timeout).build().expect("bad client build");
                self
            }

            {{~#each specPaths}}
                {{~#with get}}{{~> async_operation_fn operation_verb="get"}}{{~/with}}
                {{~#with head}}{{~> async_operation_fn operation_verb="head"}}{{~/with}}
                {{~#with post}}{{~> async_operation_fn operation_verb="post"}}{{~/with}}
                {{~#with put}}{{~> async_operation_fn operation_verb="put"}}{{~/with}}
                {{~#with delete}}{{~> async_operation_fn operation_verb="delete"}}{{~/with}}
                {{~#with options}}{{~> async_operation_fn operation_verb="options"}}{{~/with}}
                {{~#with trace}}{{~> async_operation_fn operation_verb="trace"}}{{~/with}}
                {{~#with patch}}{{~> async_operation_fn operation_verb="patch"}}{{~/with}}
            {{~/each}}
        }

        {{~#*inline "shortcut_to_data_model"}}

        pub mod {{snakecase operationId}} {
            pub use crate::models::{{snakecase specName "models"}}::{{snakecase operationId}}::*;

            #[allow(clippy::large_enum_variant)]
            #[derive(Debug, thiserror::Error, displaydoc::Display)]
            pub enum Error {
                /// Request {{snakecase operationId}} failed: {0:#}
                Client(#[from] super::ReqwestError),
                /// IO error occured while retrieving response body: {0:#}
                Io(#[from] std::io::Error),
                /// Request body serialization to JSON failed: {0:#}
                BodySerialization(#[from] serde_json::Error),
                /// Request parameters serialization failed: {0:#}
                ParametersSerialization(#[from] serde_urlencoded::ser::Error),
                /// Timeout occured during request: {0:#}
                Timeout(#[from] async_std::future::TimeoutError),
                {{~#each responses}}
                {{~#if (not (eq @key "default"))}}
                /// Status {{@key}} error: {0:?}
                {{camelcase "Status" @key}}({{camelcase "Status" @key}}),
                {{~/if}}
                {{~/each}}
                /// Unknown: {headers:?} {text:?}
                Unknown {
                    headers: reqwest::header::HeaderMap,
                    text: reqwest::Result<String>
                },
            }

            impl Error {
                pub async fn unknown(response: reqwest::Response) -> Self {
                    Self::Unknown{
                        headers: response.headers().clone(),
                        text: response.text().await,
                    }
                }
            }
        }
        {{~/inline}}

        {{~#each specPaths}}
        {{~#with get}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~#with head}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~#with post}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~#with put}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~#with delete}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~#with options}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~#with trace}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~#with patch}}{{~> shortcut_to_data_model specName=specName}}{{~/with}}
        {{~/each}}
    }
{{~/each}}
