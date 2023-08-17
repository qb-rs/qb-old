// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE
// 
// (c) Copyright 2023 The QuixByte Authors

use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(migration::Migrator).await;
}
