use crate::commons::unitily::get_db;
use crate::entities::{study, study::Model};
use migration::ExprTrait;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityOrSelect,
    EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
};
use serde_json::Value;

pub async fn get_list(index: i32, level: i32) -> Result<(Vec<Value>), DbErr> {
    let db = get_db().await?;

    let res = study::Entity::find()
        .filter(
            Condition::all()
                .add(study::Column::Level.eq(level))
                .add(study::Column::Index.gte(index - 5))
                .add(study::Column::Index.gte(index + 5)),
        )
        .order_by_asc(study::Column::Index)
        .into_json()
        .all(&db)
        .await?;
    Ok(res)
}
async fn get_model(con: Condition) -> Option<Value> {
    let db = get_db().await.unwrap();

    let res = study::Entity::find()
        .filter(con)
        .order_by_asc(study::Column::Index)
        .into_json()
        .one(&db)
        .await
        .unwrap();
    res
}
pub async fn get_model_index(level: i32) -> Option<Value> {
    get_model(Condition::all().add(study::Column::Level.eq(level))).await
}

pub async fn insert(model: study::Model) -> Result<Model, DbErr> {
    let db = get_db().await?;
    let m: study::ActiveModel = model.into();
    m.insert(&db).await
}

pub async fn update(model: study::Model) -> Result<Model, DbErr> {
    let db = get_db().await?;
    let study: study::ActiveModel = model.into();
    study.update(&db).await
}

pub async fn delete(id: i32) -> Result<u64, DbErr> {
    let db = get_db().await?;
    Ok(study::Entity::delete_by_id(id)
        .exec(&db)
        .await?
        .rows_affected)
}
