// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE
//
// (c) Copyright 2023 The QuixByte Authors

use actix_web::{get, web, App, HttpServer, Responder};
use redis::Commands;

struct State {
    pub redis: r2d2::Pool<redis::Client>,
}

#[get("/")]
async fn index(data: web::Data<State>) -> impl Responder {
    let redis: &mut r2d2::PooledConnection<redis::Client> = &mut data.redis.get().unwrap();
    let hit: i32 = redis.incr("page_hits", 1).unwrap();

    format!("Hello, World! Page hits: {}", hit)
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = redis::Client::open("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().max_size(15).build(client).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(State {
                redis: pool.clone(),
            }))
            .service(index)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
