use entity::sea_orm_active_enums as soae;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GetUser {
    pub user_uuid: Option<uuid::Uuid>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateUser {
    pub user_email: String,
    pub user_realname: String,
    pub user_password: String,
    pub user_status: soae::UserStatus,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveUser {
    pub user_uuid: uuid::Uuid,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateUser {
    pub user_password: String
}