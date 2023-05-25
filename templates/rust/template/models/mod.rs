#![allow(clippy::clone_on_copy)]

pub mod components {
    pub mod schemas {
        use super::super::components;
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use std::convert::TryFrom;

        {{~#with components}}
            {{~#each schemas}}
                {{>schema name=@key this}}
            {{~/each}}
        {{~/with}}
    }
}

{{#each paths}}
    pub mod {{snakecase @key "models"}} {
    {{#each this}}
        {{~>operation_types get noBody=true}}
        {{~>operation_types head noBody=true}}
        {{~>operation_types post}}
        {{~>operation_types put}}
        {{~>operation_types delete}}
        {{~>operation_types options}}
        {{~>operation_types trace}}
        {{~>operation_types patch}}
    {{~/each}}
    }
{{~/each}}
