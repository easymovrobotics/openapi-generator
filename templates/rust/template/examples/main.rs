use {{package_name}}::{models::*, server::*};
use actix_web::{HttpServer, App};
use async_trait::async_trait;

{{~#*inline "operation_fn_trait"}}

    async fn {{snakecase operationId}}(
        &self,
        _parameters: {{snakecase operationId}}::Parameters,
        {{#unless noBody~}} _body: {{snakecase operationId}}::Body, {{~/unless}}
    ) -> Result<{{snakecase operationId}}::Success, {{snakecase operationId}}::Error<Self::Error>> {
        unimplemented!()
    }
{{~/inline}}

#[derive(Clone)]
struct Server;

{{~#each specs}}

    #[async_trait(?Send)]
    impl {{camelcase info.title}} for Server {
        type Error = std::io::Error;
    {{~#each paths}}
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
{{~/each}}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let server = Server {};
    HttpServer::new(move || App::new().data(server.clone()).configure(config::<Server>))
    .bind("127.0.0.1:8080")?
    .run()
    .await
}