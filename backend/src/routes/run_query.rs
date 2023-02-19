use entity::sea_orm_active_enums as soae;
use migration::sea_orm as so;
use actix_web as aw;

use so::ConnectionTrait;

use crate::state::r#struct as stcState;
use common::commands::SqlQuery;

pub async fn run_query(self_user: aw::web::ReqData<entity::users_table::Model>, app_state: aw::web::Data<stcState::State>, query: aw::web::Query<SqlQuery>) -> impl aw::Responder {
    let query = app_state.database_connection.execute(so::Statement::from_string(so::DatabaseBackend::Postgres, query.query.clone())).await;

    if let soae::UserStatus::Coadministrator = self_user.user_status {
        return aw::HttpResponse::Forbidden().finish()
    }

    if let Ok(result) = query {
        return aw::HttpResponse::Ok().body(format!("{result:#?}"))
    }

    aw::HttpResponse::BadRequest().finish()
}