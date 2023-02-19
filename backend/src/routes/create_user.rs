use entity::sea_orm_active_enums as soae;
use migration::sea_orm as so;
use actix_web as aw;

use so::ActiveModelTrait;
use sha2::Digest;

use crate::state::r#struct as stcState;
use common::commands::user::CreateUser;

pub async fn create_user(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<CreateUser>) -> impl aw::Responder {
    let query = query.into_inner();

    if let soae::UserStatus::Coadministrator = self_user.user_status {
        if let soae::UserStatus::Administrator | soae::UserStatus::Coadministrator = query.user_status {
            return aw::HttpResponse::Forbidden().finish()
        }
    }

    let mut hasher = sha2::Sha256::new();

    hasher.update(query.user_password);

    let password = hasher.finalize().iter().map(|x| format!("{x:02x}")).collect::<String>();

    let active_user = entity::users_table::ActiveModel {
        user_uuid: so::ActiveValue::NotSet,
        user_email: so::ActiveValue::Set(query.user_email),
        user_status: so::ActiveValue::Set(query.user_status),
        user_realname: so::ActiveValue::Set(query.user_realname),
        user_password: so::ActiveValue::Set(password),
    };

    let insertion = active_user.insert(&app_state.database_connection).await;

    if let Ok(user) = insertion {
        return aw::HttpResponse::Ok().json(user)
    }

    aw::HttpResponse::InternalServerError().finish()
}