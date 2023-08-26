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
use argon2::password_hash::SaltString;
use argon2::PasswordHasher;
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
    Scope::new("/auth").service(login).service(register)
}

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub name: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterUser {
    pub name: String,
    pub display_name: String,
    pub password: String,
}

#[post("/login")]
async fn login(state: web::Data<State>, req: web::Json<LoginUser>) -> impl Responder {
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

    HttpResponse::Ok().json(json!({ "session": session }))
}

#[post("/register")]
async fn register(state: web::Data<State>, req: web::Json<RegisterUser>) -> impl Responder {
    if !(4..=16).contains(&req.name.len()) {
        return HttpApiProblem::new(StatusCode::BAD_REQUEST)
            .title("Invalid name")
            .detail("The name identifier should be between 4 and 16 characters long.")
            .type_url("https://quixbyte.com/errors/invalid_name")
            .instance("/auth/register")
            .to_actix_response();
    }

    for char in req.name.as_bytes().iter() {
        // 97 - 122 => a - z | 95 => _
        if (97..122).contains(char) || *char == 95 {
            continue;
        }

        return HttpApiProblem::new(StatusCode::BAD_REQUEST)
            .title("Invalid name")
            .detail("The name identifier should only contain a-z + '_'.")
            .type_url("https://quixbyte.com/errors/invalid_name")
            .instance("/auth/register")
            .to_actix_response();
    }

    if !(1..=50).contains(&req.display_name.len()) {
        return HttpApiProblem::new(StatusCode::BAD_REQUEST)
            .title("Invalid display name")
            .detail("The display name should be between 1 and 50 characters long.")
            .type_url("https://quixbyte.com/errors/invalid_display_name")
            .instance("/auth/register")
            .to_actix_response();
    }

    let password = Argon2::default()
        .hash_password(req.password.as_bytes(), &SaltString::generate(&mut OsRng))
        .unwrap()
        .to_string();

    if let Err(_) = user::Entity::insert(user::ActiveModel {
        name: sea_orm::Set(req.name.clone()),
        display_name: sea_orm::Set(req.display_name.clone()),
        password: sea_orm::Set(password),
        id: sea_orm::NotSet,
    })
    .exec(&state.db_pool)
    .await
    {
        return HttpApiProblem::new(StatusCode::CONFLICT)
            .title("User conflict")
            .detail("A user with this name identifier already exists.")
            .type_url("https://quixbyte.com/errors/user_conflict")
            .instance("/auth/register")
            .to_actix_response();
    }

    HttpResponse::Ok().body("OK")
}
