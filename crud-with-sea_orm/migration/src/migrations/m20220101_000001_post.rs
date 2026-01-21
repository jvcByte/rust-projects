use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Id))
                    .col(string(Post::Title))
                    .col(text(Post::Text))  // Use 'text' for longer content
                    .col(timestamp_with_time_zone(Post::CreatedAt))
                    .col(timestamp_with_time_zone(Post::UpdatedAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

// Define table and column names as enum
#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
    CreatedAt,
    UpdatedAt,
}
