// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE

// (c) Copyright 2023 The QuixByte Authors

use actix_web::Scope;

pub fn scope() -> Scope {
    Scope::new("/api").service(auth::scope())
}

pub mod auth;
