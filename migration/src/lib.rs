#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250626_143627_add_bio_and_image_to_users;
mod m20250626_143658_articles;
mod m20250626_143725_comments;
mod m20250626_143744_tags;
mod m20250626_143817_create_join_table_users_and_articles;
mod m20250626_143825_create_join_table_users_and_users;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250626_143627_add_bio_and_image_to_users::Migration),
            Box::new(m20250626_143658_articles::Migration),
            Box::new(m20250626_143725_comments::Migration),
            Box::new(m20250626_143744_tags::Migration),
            Box::new(m20250626_143817_create_join_table_users_and_articles::Migration),
            Box::new(m20250626_143825_create_join_table_users_and_users::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}