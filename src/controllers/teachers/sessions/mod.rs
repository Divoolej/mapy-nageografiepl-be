use actix_web::{web, Scope};

mod create;

pub fn root() -> Scope {
    web::scope("/sessions")
        .service(create::action)
}
