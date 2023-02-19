pub mod article;
pub mod user;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlQuery {
    pub query: String
}