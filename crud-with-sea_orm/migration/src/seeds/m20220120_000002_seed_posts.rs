use chrono::Utc;
use entity::post;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_data = vec![
            ("First Post", "This is the first post."),
            ("Second Post", "This is another post."),
        ];

        for (title, text) in seed_data {
            let now = Utc::now().into();
            let model = post::ActiveModel {
                title: Set(title.to_string()),
                text: Set(text.to_string()),
                created_at: Set(now),
                updated_at: Set(now),
                ..Default::default()
            };
            model.insert(db).await?;
        }

        println!("Posts table seeded successfully.");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let titles_to_delete = vec!["First Post", "Second Post"];
        post::Entity::delete_many()
            .filter(post::Column::Title.is_in(titles_to_delete))
            .exec(db)
            .await?;

        println!("Posts seeded data removed.");
        Ok(())
    }
}
