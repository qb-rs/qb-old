use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/api").service(auth::scope())
}

pub mod auth;
