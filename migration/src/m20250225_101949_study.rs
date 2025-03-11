use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Study::Table)
                    .if_not_exists()
                    .col(pk_auto(Study::Id)) //生成有bug，没有auto_increment   #[sea_orm(primary_key, auto_increment=true)]
                    .col(integer(Study::Level))
                    .col(integer(Study::Index))
                    .col(string(Study::Content))
                    .col(string(Study::A)).col(string(Study::B)).col(string(Study::C)).col(string(Study::D))
                    .col(string(Study::Remark))
                    .col(integer(Study::Result))
                    .col(integer(Study::Type))
                    .col(date_time(Study::CreateDate))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .drop_table(Table::drop().table(Study::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Study {
    Table,
    Id,
    Level,
    Index,
    Content,
    A,B,C,D,
    Remark,
    Result,
    Type,
    CreateDate,
}

