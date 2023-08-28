// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE
//
// (c) Copyright 2023 The QuixByte Authors

#[macro_use]
extern crate serde_json;

use std::env;

use actix_web::{web, App, HttpServer};
use tracing::warn;
use tracing_unwrap::ResultExt;

use qb_migration::{Migrator, MigratorTrait};

mod auth;
mod state;

pub use state::State;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    if let Err(err) = dotenv::dotenv() {
        warn!(".env was not loaded successfully: {err:?}");
    }

    let redis_url = env::var("REDIS_URL").expect_or_log("REDIS_URL env variable not set");
    let db_url = env::var("DATABASE_URL").expect_or_log("DATABASE_URL env variable not set");

    let redis_pool = r2d2::Pool::builder()
        .max_size(15)
        .build(redis::Client::open(redis_url).expect_or_log("Failed to setup redis pool"))
        .expect_or_log("Failed to setup redis pool");

    let db_pool = sea_orm::Database::connect(db_url)
        .await
        .expect_or_log("Failed to setup database pool");
    Migrator::up(&db_pool, None)
        .await
        .expect_or_log("Failed to run database migrations");

    let state = State::new(redis_pool, db_pool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(auth::scope())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
