# SPDX-License-Identifier: AGPL-3.0-only

# ████████████████████████████████████████████████
# █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
# █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
# ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
# https://github.com/QuixByte/qb/blob/main/LICENSE
# 
# (c) Copyright 2023 The QuixByte Authors

# $: docker build -t qb-frontend -f qb-frontend/Dockerfile.dev .

FROM node:alpine

WORKDIR /app/qb-frontend

COPY qb-frontend/package*.json . 
RUN npm install

COPY . ..

CMD [ "npm", "run", "dev", "--", "--host" ]
