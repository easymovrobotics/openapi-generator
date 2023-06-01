{{~#each paths }}
    #[cfg(feature = "{{snakecase @key "server"}}")]
    pub mod {{snakecase @key "security"}} {
        use actix_web::http::Method;
        use maplit::hashmap;
        use maplit::hashset;
        use once_cell::sync::Lazy;
        use std::collections::HashMap;
        use std::collections::HashSet;

        
        pub static {{shoutysnakecase @key}}_SECURITY_MATRIX: Lazy<HashMap<(&str, Method), HashSet<&str>>> = Lazy::new(|| {
            hashmap! {
            {{~#each this as | _ path |}}
                {{~#with get}}
                    ("{{path}}", Method::GET) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
                {{~#with head}}
                    ("{{path}}", Method::HEAD) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
                {{~#with post}}
                    ("{{path}}", Method::POST) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
                {{~#with put}}
                    ("{{path}}", Method::PUT) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
                {{~#with delete}}
                    ("{{path}}", Method::DELETE) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
                {{~#with options}}
                    ("{{path}}", Method::OPTIONS) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
                {{~#with trace}}
                    ("{{path}}", Method::TRACE) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
                {{~#with patch}}
                    ("{{path}}", Method::PATCH) => hashset![
                        {{~#each security}}
                            {{~#each auth as | scope |}}
                                "{{scope}}",
                            {{~/each}}
                        {{~/each}}
                    ],
                {{~/with}}
            {{~/each}}
                }
        });
    }
{{~/each}}