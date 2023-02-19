use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_type(
            Type::create()
            .as_enum(ArticleType::ArticleType)
            .values([
                ArticleType::Announcement,
                ArticleType::Important,
                ArticleType::Normal,
            ])
            .to_owned()
        ).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_type(
            Type::drop()
            .if_exists()
            .name(ArticleType::ArticleType)
            .to_owned()
        ).await
    }
}

#[derive(Iden)]
pub enum ArticleType {
    ArticleType,

    #[iden = "ANNOUNCEMENT"]
    Announcement,

    #[iden = "IMPORTANT"]
    Important,

    #[iden = "NORMAL"]
    Normal,
}