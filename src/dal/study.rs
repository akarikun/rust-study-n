use crate::commons::unitily::get_db;
use crate::entities::{study, study::Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DbErr, EntityOrSelect,
    EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,DatabaseBackend,
};
use serde_json::Value;

pub async fn get_list(index: i32, level: i32, step: i32) -> Result<Vec<Value>, DbErr> {
    let db = get_db().await?;
    let mut cond = Condition::all().add(study::Column::Level.eq(level));
    
    if step > 0 {
        cond = cond
            .add(study::Column::Index.gte(index - step))
            .add(study::Column::Index.lte(index + step))
    } else if index > 0 {
        cond = cond.add(study::Column::Index.eq(index));
    }    

    let res = study::Entity::find()
        .filter(cond)
        .order_by_asc(study::Column::Index)
        .into_json()
        .all(&db)        
        .await?;
    Ok(res)
}
pub async fn get_last_index(level: i32) -> Result<i64, DbErr> {
    let db = get_db().await?;

    let res = study::Entity::find()
        .filter(Condition::all().add(study::Column::Level.eq(level)))
        .order_by_desc(study::Column::Index)
        .into_json()
        .one(&db)
        .await?;
    if let Some(val) = res {
        let id = val["id"].as_i64().unwrap();
        Ok(id)
    } else {
        Ok(0)
    }
}

pub async fn get_model_index(level: i32) -> Option<Value> {
    let db = get_db().await.unwrap();
    let res = study::Entity::find()
        .filter(Condition::all().add(study::Column::Level.eq(level)))
        .order_by_asc(study::Column::Index)
        .into_json()
        .one(&db)
        .await
        .unwrap();
    res
}
pub async fn get_model(id: i32) -> Option<Value> {
    let db = get_db().await.unwrap();
    let res = study::Entity::find()
        .filter(Condition::all().add(study::Column::Id.eq(id)))
        .into_json()
        .one(&db)
        .await
        .unwrap();
    res
}

pub async fn insert(model: study::Model) -> Result<Model, DbErr> {
    let db = get_db().await?;
    let m: study::ActiveModel = model.into();
    m.insert(&db).await
}

pub async fn update(model: study::Model) -> Result<Model, DbErr> {
    let db = get_db().await?;
    let m: study::ActiveModel = model.into();
    m.reset_all().update(&db).await
}

pub async fn delete(id: i32) -> Result<u64, DbErr> {
    let db = get_db().await?;
    Ok(study::Entity::delete_by_id(id)
        .exec(&db)
        .await?
        .rows_affected)
}
