# SPDX-License-Identifier: AGPL-3.0-only

# ████████████████████████████████████████████████
# █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
# █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
# ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
# https://github.com/QuixByte/qb/blob/main/LICENSE

# (c) Copyright 2023 The QuixByte Authors

# $: docker build -t qb-backend -f qb-backend/Dockerfile .

FROM rust:alpine as builder

RUN apk add musl-dev pkgconfig

WORKDIR /app

COPY . .

ARG FEATURES=default

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/app/target \
  cargo build --bin qb-backend --release --features $FEATURES

RUN --mount=type=cache,target=/app/target \
  mv target/release/qb-backend /usr/bin/qb-backend

FROM scratch

COPY --from=builder /usr/bin/qb-backend /usr/bin/qb-backend

CMD [ "qb-backend" ]
