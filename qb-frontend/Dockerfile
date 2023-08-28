# SPDX-License-Identifier: AGPL-3.0-only

# ████████████████████████████████████████████████
# █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
# █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
# ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
# https://github.com/QuixByte/qb/blob/main/LICENSE
# 
# (c) Copyright 2023 The QuixByte Authors

# $: docker build -t qb-frontend -f qb-frontend/Dockerfile .

FROM node:alpine as builder

WORKDIR /app

COPY qb-frontend/package*.json .
RUN npm ci --omit=dev

COPY qb-frontend .

RUN npm run build

FROM nginx:alpine

COPY ./nginx.conf /etc/nginx/nginx.conf
COPY --from=builder /app/build /usr/share/nginx/html/
