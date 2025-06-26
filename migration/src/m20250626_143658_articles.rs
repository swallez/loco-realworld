use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "articles",
            &[
            
            ("id", ColType::PkAuto),
            
            ("title", ColType::String),
            ("description", ColType::StringNull),
            ("body", ColType::String),
            ("slug", ColType::StringUniq),
            ],
            &[
            ("user", "author_id"),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "articles").await
    }
}
