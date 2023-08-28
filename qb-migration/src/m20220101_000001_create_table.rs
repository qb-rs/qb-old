// SPDX-License-Identifier: AGPL-3.0-only

// ████████████████████████████████████████████████
// █─▄▄▄─█▄─██─▄█▄─▄█▄─▀─▄█▄─▄─▀█▄─█─▄█─▄─▄─█▄─▄▄─█
// █─██▀─██─██─███─███▀─▀███─▄─▀██▄─▄████─████─▄█▀█
// ▀───▄▄▀▀▄▄▄▄▀▀▄▄▄▀▄▄█▄▄▀▄▄▄▄▀▀▀▄▄▄▀▀▀▄▄▄▀▀▄▄▄▄▄▀
// https://github.com/QuixByte/qb/blob/main/LICENSE
//
// (c) Copyright 2023 The QuixByte Authors

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().unique_key().not_null())
                    .col(ColumnDef::new(User::DisplayName).string().not_null())
                    .col(ColumnDef::new(User::Password).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    // Used internally to identify users. An id is immutable and will **NEVER** be updated. The
    // only time that an id is invalidated is after a successful account deletion.
    Id,
    // Used to for QB's users as a readable identifier. This can be updated via the web interface
    // but is strongly ratelimited. A name identifier must only contain the lowercase letters in the latin
    // alphabeth + '_' and its length must be contained in the range 4..=14 => /^[a-z_]{4, 14}$/
    Name,
    // Used as a more customizable way for users to update their name. This can include any special
    // characters as long the length is contained in the range 1..=50 => /^.{1, 50}$/
    DisplayName,
    // Contains the digest of the argon2 hashing algorithm applied to a randomly generated salt
    // and the password the user provided when registering/resetting their password.
    Password,
}
