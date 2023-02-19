use migration::sea_orm as so;
use actix_web as aw;

use so::ActiveModelTrait;
use so::IntoActiveModel;
use sha2::Digest;

use common::commands::user::UpdateUser;
use crate::state::r#struct as stcState;

pub async fn update_user(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<UpdateUser>) -> impl aw::Responder {
    let self_user = self_user.into_inner();
    let query = query.into_inner();

    let mut active_user = self_user.into_active_model();

    let mut hasher = sha2::Sha256::new();

    hasher.update(query.user_password);

    let password = hasher.finalize().iter().map(|x| format!("{x:02x}")).collect::<String>();

    active_user.user_password = so::ActiveValue::Set(password);

    let updation = active_user.update(&app_state.database_connection).await;

    if let Ok(user) = updation {
        return aw::HttpResponse::Ok().json(user)
    }

    aw::HttpResponse::InternalServerError().finish()
}