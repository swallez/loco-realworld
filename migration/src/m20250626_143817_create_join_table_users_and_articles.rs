use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_join_table(m, "favorites",
            &[
            ],
            &[
            ("user", "user_id"),
            ("article", "article_id"),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "favorites").await
    }
}
