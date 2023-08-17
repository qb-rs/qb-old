// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE
// 
// (c) Copyright 2023 The QuixByte Authors

use tracing_unwrap::ResultExt;

#[derive(Clone)]
pub struct State {
    pub redis_pool: r2d2::Pool<redis::Client>,
    pub db_pool: sea_orm::DatabaseConnection,
}

impl State {
    pub fn new(
        redis_pool: r2d2::Pool<redis::Client>,
        db_pool: sea_orm::DatabaseConnection,
    ) -> Self {
        Self {
            redis_pool,
            db_pool,
        }
    }

    pub fn redis(&self) -> r2d2::PooledConnection<redis::Client> {
        self.redis_pool
            .get()
            .expect_or_log("receiving a connection to the redis database failed")
    }
}
