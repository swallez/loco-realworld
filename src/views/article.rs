use sea_orm::prelude::DateTimeWithTimeZone;
use serde::{Deserialize, Serialize};
use crate::models::_entities::articles;
use crate::views::auth::UserProfile;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleView {
    slug: String,
    title: String,
    description: String,
    #[serde(skip_serializing_if="Option::is_none")]
    body: Option<String>,
    tag_list: Vec<String>,
    created_at: DateTimeWithTimeZone,
    updated_at: DateTimeWithTimeZone,
    favorited: bool,
    favorites_count: i32,
    author: UserProfile,
}

impl ArticleView {
    pub fn from(article: articles::Model, author: UserProfile) -> Self {
        ArticleView {
            slug: article.slug,
            title: article.title,
            description: article.description.unwrap_or("No description".to_string()),
            body: Some(article.body),
            author: author,
            created_at: article.created_at,
            updated_at: article.updated_at,
            tag_list: Vec::new(), // TODO
            favorited: false,     // TODO
            favorites_count: 0,   // TODO
        }
    }

    pub fn compact_from(article: articles::Model, author: UserProfile) -> Self {
        // Do not return the body
        let mut view = Self::from(article, author);
        view.body = None;
        view
    }
}
