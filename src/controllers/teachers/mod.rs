mod create;

use actix_web::{web, Scope};

pub fn root() -> Scope {
    web::scope("/teachers")
        .service(create::action)
}
