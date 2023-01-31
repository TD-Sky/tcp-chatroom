use entity::{group, prelude::Group};
use sea_orm::{prelude::*, ActiveValue};

#[inline]
pub async fn batch(db: &DatabaseConnection) -> Vec<group::Model> {
    Group::find().all(db).await.unwrap()
}

pub async fn insert(
    db: &DatabaseConnection,
    owner_id: i64,
    group: group::InsertModel,
) -> Result<group::Model, DbErr> {
    use ActiveValue::*;

    let group = group::ActiveModel {
        id: NotSet,
        name: Set(group.name),
        owner_id: Set(owner_id),
    };

    group.insert(db).await
}
