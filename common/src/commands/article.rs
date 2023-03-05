use entity::sea_orm_active_enums as soae;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GetArticle {
    pub article_title: Option<String>,
    pub article_content: Option<String>,
    pub article_uuid: Option<uuid::Uuid>,
    pub article_type: Option<soae::ArticleType>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct GetArticlePrivate {
    pub article_title: Option<String>,
    pub article_content: Option<String>,
    pub article_uuid: Option<uuid::Uuid>,
    pub article_type: Option<soae::ArticleType>,
    pub article_author: Option<uuid::Uuid>,
    pub article_visibility: Option<bool>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateArticle {
    pub article_title: String,
    pub article_content: String,
    pub article_type: soae::ArticleType,
    pub article_visibility: Option<bool>,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveArticle {
    pub article_uuid: uuid::Uuid
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateArticle {
    pub article_uuid: uuid::Uuid,
    pub article_title: Option<String>,
    pub article_content: Option<String>,
    pub article_type: Option<soae::ArticleType>,
    pub article_visibility: Option<bool>
}
