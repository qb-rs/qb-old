# SPDX-License-Identifier: AGPL-3.0-only

# ████████████████████████████████████████████████
# █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
# █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
# ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
# https://github.com/QuixByte/qb/blob/main/LICENSE
# 
# (c) Copyright 2023 The QuixByte Authors

FROM node:alpine as frontend-builder

WORKDIR /app/qb-frontend

COPY qb-frontend/package*.json . 
RUN npm ci --omit=dev

COPY . ..

RUN npm run build

FROM nginx

COPY qb-proxy/nginx.conf /etc/nginx/nginx.conf
COPY --from=frontend-builder /app/qb-frontend/build/ /usr/share/nginx/html/
