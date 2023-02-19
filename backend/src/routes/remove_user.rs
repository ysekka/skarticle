use entity::sea_orm_active_enums as soae;
use migration::sea_orm as so;
use actix_web as aw;

use so::ColumnTrait;
use so::QueryFilter;
use so::EntityTrait;

use common::commands::user::RemoveUser;
use crate::state::r#struct as stcState;

pub async fn remove_user(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<RemoveUser>) -> impl aw::Responder {
    let query = query.into_inner();

    let articles_deletion = entity::articles_table::Entity::delete_many()
    .filter(entity::articles_table::Column::ArticleAuthor.eq(query.user_uuid)).exec(&app_state.database_connection).await;

    if let Ok(_) = articles_deletion {
        let mut user_deletion = entity::users_table::Entity::delete_by_id(query.user_uuid);

        if let soae::UserStatus::Coadministrator = self_user.user_status {
            user_deletion = user_deletion.filter(entity::users_table::Column::UserStatus.ne(soae::UserStatus::Administrator))
            .filter(entity::users_table::Column::UserStatus.ne(soae::UserStatus::Coadministrator));
        }

        if let Ok(result) = user_deletion.exec(&app_state.database_connection).await {
            if result.rows_affected != 0 {
                return aw::HttpResponse::Ok().finish()
            }

            return aw::HttpResponse::NotFound().finish()
        }
    }

    aw::HttpResponse::InternalServerError().finish()
}