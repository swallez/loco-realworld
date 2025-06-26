#![allow(clippy::missing_errors_doc)]
#![allow(clippy::unnecessary_struct_initialization)]
#![allow(clippy::unused_async)]

use std::collections::HashMap;
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use axum::debug_handler;
use axum::extract::Query;
use loco_rs::prelude::auth::*;
use sea_orm::{PaginatorTrait, QueryOrder, QuerySelect};
use serde_json::json;
use crate::models::_entities::articles::{ActiveModel, Entity, Model};
use crate::models::_entities::articles;
use crate::models::_entities::users;
use crate::views::article::ArticleView;
use crate::views::auth::UserProfile;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Params {
    pub title: String,
    pub description: Option<String>,
    pub body: String,
    pub slug: String,
    }

impl Params {
    fn update(&self, item: &mut ActiveModel) {
      item.title = Set(self.title.clone());
      item.description = Set(self.description.clone());
      item.body = Set(self.body.clone());
      item.slug = Set(self.slug.clone());
  }
}

async fn load_item(ctx: &AppContext, id: i32) -> Result<Model> {
    let item = Entity::find_by_id(id).one(&ctx.db).await?;
    item.ok_or_else(|| Error::NotFound)
}

#[debug_handler]
pub async fn update(
    Path(id): Path<i32>,
    State(ctx): State<AppContext>,
    Json(params): Json<Params>,
) -> Result<Response> {
    let item = load_item(&ctx, id).await?;
    let mut item = item.into_active_model();
    params.update(&mut item);
    let item = item.update(&ctx.db).await?;
    format::json(item)
}

#[debug_handler]
pub async fn remove(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    load_item(&ctx, id).await?.delete(&ctx.db).await?;
    format::empty()
}

#[debug_handler]
pub async fn get_one(Path(id): Path<i32>, State(ctx): State<AppContext>) -> Result<Response> {
    format::json(load_item(&ctx, id).await?)
}


pub fn routes() -> Routes {
    Routes::new()
        .prefix("api/articles/")
        .add("/", get(list_recent))
        .add("/", post(new_article))
        .add("{slug}", get(get_by_slug))

        .add("{slug}", delete(remove))
        .add("{slug}", put(update))
        .add("{slug}", patch(update))
}

pub async fn get_by_slug(
    State(ctx): State<AppContext>, Path(slug): Path<String>
) -> Result<Response> {

    let article = Entity::find()
        .filter(articles::Column::Slug.eq(slug))
        .one(&ctx.db).await?
        .ok_or_else(|| Error::NotFound)?;

    let author = users::Entity::find_by_id(article.author_id).one(&ctx.db).await?
        .ok_or_else(|| Error::NotFound)?;

    let response = json!({
        "article": ArticleView::from(article, UserProfile::new(author))
    });

    format::json(response)
}

#[derive(Debug, Deserialize)]
pub struct ListRecentQueryParams {
    offset: Option<u64>,
    limit: Option<u64>,
}

pub async fn list_recent(
    State(ctx): State<AppContext>,
    Query(params): Query<ListRecentQueryParams>,
) -> Result<Response> {

    // Load articles, with a max limit of 100
    let limit = params.limit.unwrap_or(10).clamp(0, 100);

    let articles = Entity::find().order_by_desc(articles::Column::CreatedAt)
        .offset(params.offset)
        .limit(limit)
        .all(&ctx.db).await?;

    // Fetch all distinct author names
    let mut user_ids = articles.iter().map(|article| article.author_id).collect::<Vec<_>>();
    user_ids.sort();
    user_ids.dedup();

    let authors = users::Entity::find()
        .filter(users::Column::Id.is_in(user_ids))
        .all(&ctx.db).await?;

    let authors: HashMap<i32, UserProfile> = authors.into_iter()
        .map(|author| (author.id, UserProfile::new(author))).collect();

    // Build eacch article's view
    let mut views = Vec::new();
    for article in articles {
        let author = authors.get(&article.author_id)
            .ok_or_else(|| Error::NotFound)?;

        views.push(ArticleView::compact_from(article, author.clone()))
    }

    // And produce the response
    let total_count = articles::Entity::find().count(&ctx.db).await?;

    let response = json!({
        "articles": views,
        "articlesCount": total_count,
    });

    format::json(response)
}


#[derive(Debug, Deserialize)]
pub struct NewArticleRequest {
    article: NewArticleParams,
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewArticleParams {
    #[validate(length(min = 1, max = 200))]
    title: String,
    description: Option<String>,
    body: String,
}

pub async fn new_article(
    State(ctx): State<AppContext>,
    auth: JWT,
    Json(new_article_request): Json<NewArticleRequest>,
) -> Result<Response> {


    let author = users::Model::find_by_pid(&ctx.db, &auth.claims.pid).await?;

    let tx = ctx.db.begin().await?;

    let param = new_article_request.article;
    param.validate()?;

    let slug = slug::slugify(&param.title);

    let article = articles::Model {
        author_id: author.id,
        slug,
        title: param.title,
        description: param.description,
        body: param.body,
        ..Default::default()
    };

    let article = article.into_active_model();
    let err = article.insert(&tx).await;
    tx.commit().await?;

    tracing::info!("Article created: {:?}", err);
    let article = err?;

    format::json(json!({
        "article": ArticleView::from(article, UserProfile::new(author))
    }))
}

