# SPDX-License-Identifier: AGPL-3.0-only

# ████████████████████████████████████████████████
# █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
# █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
# ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
# https://github.com/QuixByte/qb/blob/main/LICENSE
# 
# (c) Copyright 2023 The QuixByte Authors

# $: docker build -t qb-backend -f qb-backend/Dockerfile .

FROM rust:alpine as builder

RUN apk add musl-dev pkgconfig

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/cargo-watch \
  cargo install cargo-watch --root cargo-watch

WORKDIR /app

COPY . .

ARG FEATURES=default


CMD [ "cargo", "watch", "-x", "run", "--features", $FEATURES ]
