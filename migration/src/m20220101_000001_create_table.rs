use sea_orm_migration::prelude::*;
use sea_orm::sea_query::Table;
use entity::nostr::event;
use entity::nostr::tag;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20220101_000001_create_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(
                Table::create()
                .table(event::Entity).if_not_exists()
                .col(ColumnDef::new(event::Column::Id).string_len(65).primary_key())
                .col(ColumnDef::new(event::Column::Pubkey).string_len(65).not_null())
                .col(ColumnDef::new(event::Column::CreatedAt).big_integer().not_null())
                .col(ColumnDef::new(event::Column::Kind).tiny_unsigned().not_null())
                .col(ColumnDef::new(event::Column::Content).text().not_null())
                .col(ColumnDef::new(event::Column::Sig).string_len(129).not_null())
                .to_owned()
            ).await?;
        manager.create_table(
                Table::create()
                .table(tag::Entity).if_not_exists()
                .col(ColumnDef::new(tag::Column::Id).big_integer().primary_key())
                .col(ColumnDef::new(tag::Column::EventId).string_len(65).not_null())
                .col(ColumnDef::new(tag::Column::TagType).string_len(1))
                .col(ColumnDef::new(tag::Column::Hex).string().not_null())
                .col(ColumnDef::new(tag::Column::Data).text().not_null())
                .to_owned()
            ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop().table(event::Entity).to_owned()
            ).await
    }
}
