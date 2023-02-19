use entity::sea_orm_active_enums as soae;
use actix_web_httpauth as awh;
use migration::sea_orm as so;
use serde_json as sj;
use actix_web as aw;


use sha2::Digest;
use hmac::Mac;

use jwt::VerifyWithKey;
use aw::HttpMessage;
use so::EntityTrait;
use so::QueryFilter;
use so::ColumnTrait;

use crate::state::r#struct as stcState;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct JwtClaims {
    pub user_uuid: uuid::Uuid,
    pub user_password: String,
}

pub async fn normal_authentication(service_request: aw::dev::ServiceRequest, credentials: awh::extractors::bearer::BearerAuth) -> Result<aw::dev::ServiceRequest, (aw::Error, aw::dev::ServiceRequest)> {
    let secret_path = std::path::Path::new(env!("HOME"))
    .join(".config")
    .join("skarticle")
    .join("password");

    let secret_bytes = std::fs::read(&secret_path)
    .expect("Error occured during reading secret file.");

    let secret_value = secret_bytes.iter().map(|x| format!("{x:02x}")).collect::<String>();

    let jwt_key = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret_value.as_bytes())
    .expect("Error occured during creation of jwt key.");

    let token = credentials.token();

    let jwt_claims = token.verify_with_key(&jwt_key) as Result<JwtClaims, jwt::Error>;

    match jwt_claims {
        Ok(claims) => {
            let mut hasher = sha2::Sha256::new();
            hasher.update(claims.user_password);
            let hashed_password = hasher.finalize().iter().map(|x| format!("{x:02x}")).collect::<String>();

            let app_state = service_request.app_data::<aw::web::Data<stcState::State>>().unwrap();

            let user_query = entity::users_table::Entity::find_by_id(claims.user_uuid)
            .filter(entity::users_table::Column::UserPassword.eq(hashed_password))
            .one(&app_state.database_connection).await;

            if let Ok(Some(user)) = user_query {
                service_request.extensions_mut().insert(user);

                return Ok(service_request)
            }

            Err((aw::error::ErrorUnauthorized(sj::json!({})), service_request))
        },
        Err(_) => Err((aw::error::ErrorBadRequest(sj::json!({})), service_request))
    }
}

pub async fn administration_authentication(service_request: aw::dev::ServiceRequest, credentials: awh::extractors::bearer::BearerAuth) -> Result<aw::dev::ServiceRequest, (aw::Error, aw::dev::ServiceRequest)> {
    let secret_path = std::path::Path::new(env!("HOME"))
    .join(".config")
    .join("skarticle")
    .join("password");

    let secret_bytes = std::fs::read(&secret_path)
    .expect("Error occured during reading secret file.");

    let secret_value = secret_bytes.iter().map(|x| format!("{x:02x}")).collect::<String>();

    let jwt_key = hmac::Hmac::<sha2::Sha256>::new_from_slice(secret_value.as_bytes())
    .expect("Error occured during creation of jwt key.");

    let token = credentials.token();

    let jwt_claims = token.verify_with_key(&jwt_key) as Result<JwtClaims, jwt::Error>;

    match jwt_claims {
        Ok(claims) => {
            let mut hasher = sha2::Sha256::new();
            hasher.update(claims.user_password);
            let hashed_password = hasher.finalize().iter().map(|x| format!("{x:02x}")).collect::<String>();

            let app_state = service_request.app_data::<aw::web::Data<stcState::State>>().unwrap();

            let user_query = entity::users_table::Entity::find_by_id(claims.user_uuid)
            .filter(entity::users_table::Column::UserStatus.eq(soae::UserStatus::Administrator).or(
                entity::users_table::Column::UserStatus.eq(soae::UserStatus::Coadministrator)
            ))
            .filter(entity::users_table::Column::UserPassword.eq(hashed_password))
            .one(&app_state.database_connection).await;

            if let Ok(Some(user)) = user_query {
                service_request.extensions_mut().insert(user);

                return Ok(service_request)
            }

            Err((aw::error::ErrorUnauthorized(sj::json!({})), service_request))
        },
        Err(_) => Err((aw::error::ErrorBadRequest(sj::json!({})), service_request))
    }
}