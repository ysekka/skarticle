use migration::sea_orm as so;
use actix_web as aw;

use so::ActiveModelTrait;
use so::IntoActiveModel;
use std::io::Write;

use common::commands::article::CreateArticle;
use crate::state::r#struct as stcState;

pub async fn create_article(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<CreateArticle>, thumbnail: Option<aw::web::Bytes>) -> impl aw::Responder {
    let query = query.into_inner();

    let active_article = entity::articles_table::ActiveModel {
        article_uuid: so::ActiveValue::NotSet,
        article_timestamp: so::ActiveValue::NotSet,
        article_visibility: match query.article_visibility {
            Some(visibility) => so::ActiveValue::Set(visibility),
            None => so::ActiveValue::NotSet,
        },
        article_type: so::ActiveValue::Set(query.article_type),
        article_title: so::ActiveValue::Set(query.article_title),
        article_content: so::ActiveValue::Set(query.article_content),
        article_author: so::ActiveValue::Set(self_user.user_uuid),
        article_thumbnail: so::ActiveValue::Set(app_state.default_thumbnail.clone())
    };

    let insertion = active_article.insert(&app_state.database_connection).await;

    if let Ok(article) = insertion {
        if let Some(thumbnail) = thumbnail {
            let article_uuid = article.article_uuid;
            let thumbnail_path = std::path::Path::new(&app_state.public_directory)
            .join("thumbnail")
            .join(article_uuid.to_string());

            if let Ok(mut file) = std::fs::File::create(&thumbnail_path) {
                if let Ok(_) = file.write_all(&thumbnail) {
                    let mut active_article = article.clone().into_active_model();

                    active_article.article_thumbnail = so::ActiveValue::Set(format!("/public/thumbnail/{article_uuid}"));

                    let updation = active_article.update(&app_state.database_connection).await;

                    if let Ok(article) = updation {
                        return aw::HttpResponse::Ok().json(article)
                    }

                }

                std::fs::remove_file(&thumbnail_path)
                .expect("Error occured during deletion of thumbnail path.");
            }

            return aw::HttpResponse::InternalServerError().finish()
        }

        return aw::HttpResponse::Ok().json(article)
    }

    aw::HttpResponse::InternalServerError().finish()
}