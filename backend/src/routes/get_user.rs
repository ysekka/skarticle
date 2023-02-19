use entity::sea_orm_active_enums as soae;
use migration::sea_orm as so;
use actix_web as aw;

use so::EntityTrait;
use so::QueryFilter;
use so::ColumnTrait;

use crate::state::r#struct as stcState;
use common::commands::user::GetUser;

pub async fn get_self(self_user: aw::web::ReqData<entity::users_table::Model>) -> impl aw::Responder {
    aw::HttpResponse::Ok().json(self_user.into_inner())
}

pub async fn get_user(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<GetUser>) -> Option<impl aw::Responder> {
    let query = query.into_inner();

    let mut user_query = entity::users_table::Entity::find();

    if let soae::UserStatus::Coadministrator = self_user.user_status {
        user_query = user_query.filter(entity::users_table::Column::UserStatus.ne(soae::UserStatus::Administrator))
    }

    match query.user_uuid {
        Some(user_uuid) => {
            user_query = user_query.filter(entity::users_table::Column::UserUuid.eq(user_uuid));
    
            if let Ok(Some(user)) = user_query.one(&app_state.database_connection).await {
                return Some(aw::HttpResponse::Ok().json(user))
            }
        },

        None => {
            if let Ok(users) = user_query.all(&app_state.database_connection).await {
                return Some(aw::HttpResponse::Ok().json(users))
            }
        }
    }

    None
}