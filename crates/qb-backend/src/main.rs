// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE
//
// (c) Copyright 2023 The QuixByte Authors

use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use redis::Commands;
use tracing::warn;
use tracing_unwrap::ResultExt;

use qb_migration::{Migrator, MigratorTrait};

struct State {
    pub redis_pool: r2d2::Pool<redis::Client>,
    pub db_pool: sea_orm::DatabaseConnection,
}

#[get("/")]
async fn index(data: web::Data<State>) -> impl Responder {
    let redis: &mut r2d2::PooledConnection<redis::Client> = &mut data.redis_pool.get().unwrap();
    let hit: i32 = redis.incr("page_hits", 1).unwrap();

    format!("Hello, World! Page hits: {}", hit)
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                redis_pool: redis_pool.clone(),
                db_pool: db_pool.clone(),
            }))
            .service(index)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
