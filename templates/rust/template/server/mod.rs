#![allow(clippy::unit_arg, clippy::redundant_clone)]

{{~#each paths as |specPaths specName|}}
#[cfg(feature = "{{snakecase specName "server"}}")]
pub mod {{snakecase specName}} {
    use crate::models::{{snakecase specName "models"}}::*;
    use actix_multipart::Multipart;
    use actix_web::{web::*, Responder, HttpResponse, dev::HttpResponseBuilder, http::StatusCode};
    use async_trait::async_trait;
    use std::error::Error;
    use futures::{StreamExt, TryStreamExt};
    use std::collections::HashMap;
    use std::convert::TryFrom;

    {{~#*inline "operation_fn_trait"}}

        async fn {{snakecase operationId}}(
            &self,
            _parameters: {{snakecase operationId}}::Parameters,
            {{#unless noBody~}} _body: {{snakecase operationId}}::Body, {{~/unless}}
        ) -> Result<{{snakecase operationId}}::Success, {{snakecase operationId}}::Error<Self::Error>>;
    {{~/inline}}

    #[async_trait(?Send)]
    pub trait {{camelcase specName}} {
        type Error: std::error::Error;
    {{~#each specPaths}}
        {{~#with get}}{{~> operation_fn_trait noBody=true}}{{~/with}}
        {{~#with head}}{{~> operation_fn_trait noBody=true}}{{~/with}}
        {{~#with post}}{{~> operation_fn_trait}}{{~/with}}
        {{~#with put}}{{~> operation_fn_trait}}{{~/with}}
        {{~#with delete}}{{~> operation_fn_trait}}{{~/with}}
        {{~#with options}}{{~> operation_fn_trait}}{{~/with}}
        {{~#with trace}}{{~> operation_fn_trait}}{{~/with}}
        {{~#with patch}}{{~> operation_fn_trait}}{{~/with}}
    {{~/each}}
    }

    {{#*inline "operation_fn"}}
    {{#if summary}}/// {{summary}}{{/if}}
    {{~#if description}}/// {{description}}{{/if}}
    async fn {{snakecase operationId}}<Server: {{camelcase title}}>(
        server: Data<Server>,{{!-- {{~#if parameters}} --}}
        {{~#if (has parameters "in" "query")~}}
        query: Query<{{snakecase operationId}}::Query>,
        {{~/if}}
        {{~#if (has parameters "in" "path")~}}
        path: Path<{{snakecase operationId}}::Path>,
        {{~/if}}
        {{~#if (has parameters "in" "header")~}}
        header: {{snakecase operationId}}::Header,
        {{~/if}}

        {{~#if (and requestBody (not noBody))}}
            {{~#with requestBody.content.[application/json]}}
                body: Json<{{snakecase ../operationId}}::Body>,
            {{~/with}}
            {{~#with requestBody.content.[multipart/form-data]}}
                mut payload: Multipart,
            {{~/with}}
        {{~/if}}
    ) -> impl Responder {
        use {{snakecase operationId}}::*;
        let parameters = Parameters::new(
            {{~#if (has parameters "in" "query")~}}query.into_inner(),{{~/if}}
            {{~#if (has parameters "in" "path")~}}path.into_inner(),{{~/if}}
            {{~#if (has parameters "in" "header")~}}header,{{~/if}}
        );
        {{~#unless noBody}}
            {{~#if requestBody}}

                {{~#with requestBody.content.[application/json]}}
                    let body = body.into_inner();
                {{~/with}}

                {{~#with requestBody.content.[multipart/form-data]}}
                    let mut data = HashMap::new();

                    while let Ok(Some(mut field)) = payload.try_next().await {
                        let content_disposition = field.content_disposition().unwrap();
                        let field_name = content_disposition.get_name().unwrap().to_string();
                        let mut buffer = vec![];
                        while let Some(chunk) = field.next().await {
                            buffer.extend_from_slice(chunk.unwrap().as_ref());
                        }
                        data.insert(
                            field_name,
                            buffer,
                        );
                    }
                    let body = match {{snakecase ../operationId}}::Body::try_from(data) {
                        Ok(body) => body,
                        Err(err) => return HttpResponse::InternalServerError().body(err)
                    };
                {{~/with}}

            {{~else~}}
                let body = {{snakecase operationId}}::Body {};
            {{~/if}}
        {{~/unless}}

        match server.{{snakecase operationId}}(parameters {{~#unless noBody}}, body{{/unless}}).await {
            {{~#each responses}}
                {{~#if (not (eq @key "default"))}}
                    {{~#if (is_http_code_success @key)}}
                        {{~#if content}}

                            {{~#with content.[image/png]}}
                                Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("image/png").body(response),
                            {{~/with}}

                            {{~#with content.[image/jpeg]}}
                                Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("image/jpeg").body(response),
                            {{~/with}}

                            {{~#with content.[application/bag]}}
                                Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("application/bag").body(response),
                            {{~/with}}

                            {{~#with content.[application/pbstream]}}
                                Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("application/pbstream").body(response),
                            {{~/with}}

                            {{~#with content.[image/x-portable-graymap]}}
                                Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("image/x-portable-graymap").body(response),
                            {{~/with}}

                            {{~#with content.[text/plain]}}
                                Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("text/plain").body(response),
                            {{~/with}}

                            {{~#with content.[application/json]}}
                                Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).json(response),
                            {{~/with}}

                        {{~else~}}
                            Ok(Success::{{camelcase "Status" @key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@key}}).unwrap()).json(response),
                        {{~/if}}
                    {{~else~}}
                        {{~#if content}}
                            {{~#with content.[text/plain]}}
                                Err(Error::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("text/plain").body(response),
                            {{~/with}}

                            {{~#with content.[application/json]}}
                                Err(Error::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).json(response),
                            {{~/with}}

                        {{~else~}}
                            Err(Error::{{camelcase "Status" @key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@key}}).unwrap()).json(response),
                        {{~/if}}
                    {{~/if}}
                {{~/if}}
            {{~/each}}
            Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
        }
    }
    {{~/inline}}

    fn err_to_string(err: &dyn std::error::Error) -> String {
        let mut errors_str = Vec::new();
        let mut current_err = err.source();
        while let Some(err) = current_err {
            errors_str.push(err.to_string());
            current_err = err.source();
        }
        format!("error: {}\n\ncaused by:\n\t{}", err, errors_str.as_slice().join("\n\t"))
    }

    {{#each specPaths}}
        {{~#with get}}{{~> operation_fn title=specName noBody=true}}{{~/with}}
        {{~#with head}}{{~> operation_fn title=specName noBody=true}}{{~/with}}
        {{~#with post}}{{~> operation_fn title=specName}}{{~/with}}
        {{~#with put}}{{~> operation_fn title=specName}}{{~/with}}
        {{~#with delete}}{{~> operation_fn title=specName}}{{~/with}}
        {{~#with options}}{{~> operation_fn title=specName}}{{~/with}}
        {{~#with trace}}{{~> operation_fn title=specName}}{{~/with}}
        {{~#with patch}}{{~> operation_fn title=specName}}{{~/with}}
    {{~/each}}

    pub fn config<Server: {{camelcase @key}} + Send + Sync + Clone + 'static>(
        app: &mut ServiceConfig,
    ) {
        use actix_web::web::*;
        app
        {{~#each specPaths}}
            .service(
                resource("{{@key}}")
                    {{~#with get}}
                    .route(get().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
                    {{~#with head}}
                    .route(head().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
                    {{~#with post}}
                    .route(post().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
                    {{~#with put}}
                    .route(put().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
                    {{~#with delete}}
                    .route(delete().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
                    {{~#with options}}
                    .route(options().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
                    {{~#with trace}}
                    .route(trace().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
                    {{~#with patch}}
                    .route(patch().to({{snakecase operationId}}::<Server>))
                    {{~/with}}
            )
        {{~/each}};
    }
}
{{~/each}}