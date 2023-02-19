use migration::sea_orm as so;
use actix_web as aw;

use common::commands::article::{
    GetArticle,
    GetArticlePrivate
};
use crate::state::r#struct as stcState;

use so::EntityTrait;
use so::QueryFilter;
use so::QuerySelect;
use so::ColumnTrait;

pub async fn get_article(app_state: aw::web::Data<stcState::State>, query: aw::web::Query<GetArticle>) -> Option<impl aw::Responder> {
    let query = query.into_inner();

    println!("{:?}", "test");

    match query.article_uuid {
        Some(article_uuid) => {
            let article_query = entity::articles_table::Entity::find_by_id(article_uuid)
            .filter(entity::articles_table::Column::ArticleVisibility.eq(true))
            .select_only()
            .columns([
                entity::articles_table::Column::ArticleType,
                entity::articles_table::Column::ArticleUuid,
                entity::articles_table::Column::ArticleTitle,
                entity::articles_table::Column::ArticleContent,
                entity::articles_table::Column::ArticleThumbnail,
                entity::articles_table::Column::ArticleTimestamp,
            ])
            .into_json()
            .one(&app_state.database_connection).await;

            if let Ok(Some(article)) = article_query {
                return Some(aw::HttpResponse::Ok().json(article))
            }

            None
        },

        None => {
            let mut article_query = entity::articles_table::Entity::find()
            .filter(entity::articles_table::Column::ArticleVisibility.eq(true))
            .select_only()
            .columns([
                entity::articles_table::Column::ArticleType,
                entity::articles_table::Column::ArticleUuid,
                entity::articles_table::Column::ArticleTitle,
                entity::articles_table::Column::ArticleContent,
                entity::articles_table::Column::ArticleThumbnail,
                entity::articles_table::Column::ArticleTimestamp,
            ]);

            if let Some(article_title) = query.article_title {
                article_query = article_query.filter(entity::articles_table::Column::ArticleTitle.contains(&article_title))
            }

            if let Some(article_content) = query.article_content {
                article_query = article_query.filter(entity::articles_table::Column::ArticleContent.contains(&article_content))
            }

            if let Some(article_type) = query.article_type {
                article_query = article_query.filter(entity::articles_table::Column::ArticleType.eq(article_type))
            }

            let article_query = article_query.into_json().all(&app_state.database_connection).await;

            if let Ok(articles) = article_query {
                return Some(aw::HttpResponse::Ok().json(articles))
            }

            None
        }
    }
}

pub async fn get_article_all(app_state: aw::web::Data<stcState::State>, query: aw::web::Query<GetArticlePrivate>) -> Option<impl aw::Responder> {
    let query = query.into_inner();

    match query.article_uuid {
        Some(article_uuid) => {
            let article_query = entity::articles_table::Entity::find_by_id(article_uuid)
            .one(&app_state.database_connection).await;

            if let Ok(Some(article)) = article_query {
                return Some(aw::HttpResponse::Ok().json(article))
            }

            None
        },

        None => {
            let mut article_query = entity::articles_table::Entity::find();

            if let Some(article_title) = query.article_title {
                article_query = article_query.filter(entity::articles_table::Column::ArticleTitle.contains(&article_title))
            }

            if let Some(article_content) = query.article_content {
                article_query = article_query.filter(entity::articles_table::Column::ArticleContent.contains(&article_content))
            }

            if let Some(article_type) = query.article_type {
                article_query = article_query.filter(entity::articles_table::Column::ArticleType.eq(article_type))
            }

            if let Some(article_author) = query.article_author {
                article_query = article_query.filter(entity::articles_table::Column::ArticleAuthor.eq(article_author))
            }

            if let Some(article_visibility) = query.article_visibility {
                article_query = article_query.filter(entity::articles_table::Column::ArticleAuthor.eq(article_visibility))
            }

            let article_query = article_query.all(&app_state.database_connection).await;

            if let Ok(articles) = article_query {
                return Some(aw::HttpResponse::Ok().json(articles))
            }

            None
        }
    }
}
