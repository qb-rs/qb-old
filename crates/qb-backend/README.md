```
  ___        _      ____        _
 / _ \ _   _(___  _| __ ) _   _| |_ ___
| | | | | | | \ \/ |  _ \| | | | __/ _ \
| |_| | |_| | |>  <| |_) | |_| | ||  __/
 \__\_\\__,_|_/_/\_|____/ \__, |\__\___|
                          |___/

-- QuixByte Backend (QBB)

This crate contains the code for the backend
that runs QB. It depends on the qb-migration
and qb-entity crates, which contain code for
database management.

-- Primary Database 

QBB provides support for three primary database
options consisting of PostgreSQL, MySQL/MariaDB
and SQLite. One is required to setup one of these.

QBB should be compiled and run with the corresponding
features CLI flag to include the required database driver.

|  database       |  features flag        |
|-----------------------------------------|
|  PostgreSQL     |  --features postgres  |
|  MySQL/MariaDB  |  --features mysql     |
|  SQLITE         |  --features sqlite    |


-- Secondary Database

QBB also requires a secondary cache database,
in this case redis. You are required to setup
this database, the backend will not start otherwise.

-- Configuring

QBB is configurable via the environment variables.
It is recommended to copy the default configuration
file for either development (dev.env) or production
(prod.env) to the .env destination and then make
changes accordingly.

--

https://github.com/QuixByte/qb/blob/main/LICENSE
(c) Copyright 2023 The QuixByte Authors
```
