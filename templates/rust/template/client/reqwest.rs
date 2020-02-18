#![allow(clippy::ptr_arg)]
use crate::client::{ {{camelcase info.title}}, Response, Error};
use crate::models::*;
use url::{Url};
use std::sync::Arc;
use std::time::Duration;


#[derive(Clone)]
pub struct {{camelcase info.title "Client"}} {
    pub url: Url,
    pub client: reqwest::Client,
}

{{~#*inline "async_operation_fn"}}

    pub async fn {{snakecase operationId}}(
        &self,
        {{~#if ../parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody~}} body: &{{snakecase operationId}}::Body,{{/if~}}
    ) -> Result<{{snakecase operationId}}::Response<reqwest::Response>, Error> {
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
            .json(&body)
            {{~/if}}
            .send().await?;
        use {{snakecase operationId}}::Response::*;
        Ok(
            match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                {{~#if (eq @key "204")}}
                "{{@key}}" => {
                    log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
{{#if ../parameters~}}parameters:{:?}{{/if}}
{{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                        {{#if ../parameters~}}, parameters{{/if}}
                        {{#if ../requestBody~}}, body{{/if~}}
                    );
                    {{camelcase "Response" @key}}(())
                }
                {{~else~}}
                "{{@key}}" => {
                    let response_body = response.json().await?;
                    log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
{{#if ../parameters~}}parameters:{:?}{{/if}}
{{#if ../requestBody~}}requestBody:{:?}{{/if}}
response ({{@key}}):{:?}"#
                        {{#if ../parameters~}}, parameters{{/if}}
                        {{#if ../requestBody~}}, body{{/if~}}
                        , response_body
                    );
                    {{camelcase "Response" @key}}(response_body)
                }
                {{~/if}}
            {{~/if}}
            {{~/each}}
                _ => {
                    log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
{{#if ../parameters~}}parameters:{:?}{{/if}}
{{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                        {{#if ../parameters~}}, parameters{{/if}}
                        {{#if ../requestBody~}}, body{{/if~}}
                    );
                    Unspecified(response)
                },
        })
    }
{{~/inline}}

impl {{camelcase info.title "Client"}} {
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

    {{~#each paths}}
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

// blocking

pub mod blocking {
    use crate::client::{ {{camelcase info.title}}, Response, Error};
    use crate::models::*;
    use url::{Url};
    use std::sync::Arc;
    use std::time::Duration;

    #[derive(Clone)]
    pub struct {{camelcase info.title "Client"}} {
        pub url: Url,
        pub client: reqwest::blocking::Client,
    }

    {{~#*inline "operation_fn"}}

        fn {{snakecase operationId}}(
            &self,
            {{~#if ../parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
            {{~#if requestBody~}} body: &{{snakecase operationId}}::Body,{{/if~}}
        ) -> Result<{{snakecase operationId}}::Response<Response>, Error> {
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
                .json(&body)
                {{~/if}}
                .send()?;
            use {{snakecase operationId}}::Response::*;
            Ok(
                match response.status().as_str() {
                {{~#each responses}}
                {{~#if (not (eq @key "default"))}}
                    {{~#if (eq @key "204")}}
                    "{{@key}}" => {
                        log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
    {{#if ../parameters~}}parameters:{:?}{{/if}}
    {{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                            {{#if ../parameters~}}, parameters{{/if}}
                            {{#if ../requestBody~}}, body{{/if~}}
                        );
                        {{camelcase "Response" @key}}(())
                    }
                    {{~else~}}
                    "{{@key}}" => {
                        let response_body = response.json()?;
                        log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
    {{#if ../parameters~}}parameters:{:?}{{/if}}
    {{#if ../requestBody~}}requestBody:{:?}{{/if}}
    response ({{@key}}):{:?}"#
                            {{#if ../parameters~}}, parameters{{/if}}
                            {{#if ../requestBody~}}, body{{/if~}}
                            , response_body
                        );
                        {{camelcase "Response" @key}}(response_body)
                    }
                    {{~/if}}
                {{~/if}}
                {{~/each}}
                    _ => {
                        log::debug!(r#"
call to {{snakecase ../operationId}} ({{shoutysnakecase ../operation_verb}})
    {{#if ../parameters~}}parameters:{:?}{{/if}}
    {{#if ../requestBody~}}requestBody:{:?}{{/if}}"#
                            {{#if ../parameters~}}, parameters{{/if}}
                            {{#if ../requestBody~}}, body{{/if~}}
                        );
                        Unspecified(response)
                    },
            })
        }
    {{~/inline}}

    impl {{camelcase info.title "Client"}} {
        pub fn new(url: &str) -> Self {
            let url = Url::parse(url).expect("cannot parse url");
            Self {
                url,
                client: reqwest::blocking::Client::new(),
            }
        }

        pub fn with_timeout(mut self, timeout: Duration) -> Self {
            self.client = reqwest::blocking::Client::builder().timeout(timeout).build().expect("bad client build");
            self
        }
    }

    impl {{camelcase info.title}} for {{camelcase info.title "Client"}} {
        {{~#each paths}}
            {{~#with get}}{{~> operation_fn operation_verb="get"}}{{~/with}}
            {{~#with head}}{{~> operation_fn operation_verb="head"}}{{~/with}}
            {{~#with post}}{{~> operation_fn operation_verb="post"}}{{~/with}}
            {{~#with put}}{{~> operation_fn operation_verb="put"}}{{~/with}}
            {{~#with delete}}{{~> operation_fn operation_verb="delete"}}{{~/with}}
            {{~#with options}}{{~> operation_fn operation_verb="options"}}{{~/with}}
            {{~#with trace}}{{~> operation_fn operation_verb="trace"}}{{~/with}}
            {{~#with patch}}{{~> operation_fn operation_verb="patch"}}{{~/with}}
        {{~/each}}
    }
}

