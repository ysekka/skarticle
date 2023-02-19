use entity::sea_orm_active_enums as soae;
use migration::sea_orm as so;
use actix_web as aw;

use so::ColumnTrait;
use so::QueryFilter;
use so::EntityTrait;

use common::commands::article::RemoveArticle;
use crate::state::r#struct as stcState;

pub async fn remove_article(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<RemoveArticle>) -> impl aw::Responder {
    let query = query.into_inner();

    let article_query = entity::articles_table::Entity::find_by_id(query.article_uuid)
    .one(&app_state.database_connection).await;

    if let Ok(Some(article)) = article_query {
        let thumbnail_path = std::path::Path::new(&app_state.public_directory)
        .join("thumbnail")
        .join(article.article_uuid.to_string());

        let mut article_deletion = entity::articles_table::Entity::delete_by_id(article.article_uuid);

        if let soae::UserStatus::Author = self_user.user_status {
            article_deletion = article_deletion.filter(entity::articles_table::Column::ArticleAuthor.eq(self_user.user_uuid))
        }

        if article.article_thumbnail != "/public/thumbnail/default.svg" {
            std::fs::remove_file(&thumbnail_path)
            .expect("Error occured during removing thumbnail file.");
        }

        if let Ok(deletion) = article_deletion.exec(&app_state.database_connection).await {
            if deletion.rows_affected != 0 {
                return aw::HttpResponse::Ok().finish()
            }

            return aw::HttpResponse::NotFound().finish()
        }

        return aw::HttpResponse::InternalServerError().finish()
    }

    aw::HttpResponse::NotFound().finish()
}