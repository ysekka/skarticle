use entity::sea_orm_active_enums as soae;
use migration::sea_orm as so;
use actix_web as aw;

use so::ActiveModelTrait;
use so::IntoActiveModel;
use so::ColumnTrait;
use so::QueryFilter;
use so::EntityTrait;

use common::commands::article::UpdateArticle;
use crate::state::r#struct as stcState;

pub async fn update_article(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<UpdateArticle>) -> impl aw::Responder {
    let query = query.into_inner();

    let mut article_query = entity::articles_table::Entity::find_by_id(query.article_uuid);

    if let soae::UserStatus::Author = self_user.user_status {
        article_query = article_query.filter(entity::articles_table::Column::ArticleAuthor.eq(self_user.user_uuid));
    }

    if let Ok(Some(article)) = article_query.one(&app_state.database_connection).await {
        let mut active_article = article.into_active_model();

        if let Some(article_title) = query.article_title {
            active_article.article_title = so::ActiveValue::Set(article_title)
        }

        if let Some(article_content) = query.article_content {
            active_article.article_content = so::ActiveValue::Set(article_content)
        }

        if let Some(article_type) = query.article_type {
            active_article.article_type = so::ActiveValue::Set(article_type)
        }
        
        if let Some(article_visibility) = query.article_visibility {
            active_article.article_visibility = so::ActiveValue::Set(article_visibility)
        }

        let updation = active_article.update(&app_state.database_connection).await;

        if let Ok(article) = updation {
            return aw::HttpResponse::Ok().json(article)
        }

        return aw::HttpResponse::InternalServerError().finish()
    }

    aw::HttpResponse::NotFound().finish()
}
