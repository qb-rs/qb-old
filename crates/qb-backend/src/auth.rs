// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE
//
// (c) Copyright 2023 The QuixByte Authors

use actix_web::HttpResponse;
use actix_web::{post, web, Responder, Scope};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use http_api_problem::HttpApiProblem;
use http_api_problem::StatusCode;
use lazy_static::lazy_static;
use rand::distributions::{Alphanumeric, DistString};
use rand_core::OsRng;
use redis::Commands;
use sea_orm::ColumnTrait;
use sea_orm::{EntityTrait, QueryFilter};
use serde::Deserialize;
use tracing_unwrap::ResultExt;

use qb_entity::user;

use crate::State;

lazy_static! {
    // DUMMY_HASH is the password 'dummy_hash' with the salt 'dummy_hash' encoded using argon2
    pub static ref DUMMY_HASH: PasswordHash<'static> =
        PasswordHash::new("$argon2id$v=19$m=19456,t=2,p=1$ZHVtbXlfaGFzaA$mCwZQ8j8A6/Qq1AidH6RWqRkOEBs3BFo2P+WIEzUK9s")
        .unwrap_or_log();
}

pub fn scope() -> Scope {
    Scope::new("/auth").service(login)
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    pub name: String,
    pub user_name: String,
    pub password: String,
}

#[post("/login")]
async fn login<'a>(state: web::Data<State>, req: web::Json<LoginUser>) -> impl Responder {
    let user = user::Entity::find()
        .filter(user::Column::Name.eq(req.name.as_str()))
        .one(&state.db_pool)
        .await
        .unwrap_or_log();

    // We use a dummy hash to provide safety against timing attacks for leaking weather certain
    // users are registered or wether they are not.
    let password = match user {
        Some(ref user) => PasswordHash::new(user.password.as_str()).unwrap_or_log(),
        // TODO: remove unnessecary clone
        _ => DUMMY_HASH.clone(),
    };

    if !Argon2::default()
        .verify_password(req.password.as_bytes(), &password)
        .is_ok_and(|_| user.is_some())
    {
        return HttpApiProblem::new(StatusCode::BAD_REQUEST)
            .title("Invalid credentials")
            .detail("The name identifier and/or password you passed could not be associated with an account.")
            .type_url("https://quixbyte.com/errors/invalid_credentials")
            .instance("/auth/login")
            .to_actix_response();
    }

    let session = Alphanumeric.sample_string(&mut OsRng, 16);

    // session tokens expire within 4 hours
    let _: () = state
        .redis()
        .set_ex(format!("session:{session}"), user.unwrap().id, 4 * 60 * 60)
        .unwrap_or_log();

    HttpResponse::Ok().json(web::Json(json!({ "session": session })))
}
