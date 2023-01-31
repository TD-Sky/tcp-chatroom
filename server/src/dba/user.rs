use entity::{prelude::User, user};
use sea_orm::{prelude::*, ActiveValue, DatabaseConnection, QuerySelect};

#[derive(Debug, Clone, Copy, DeriveColumn, EnumIter)]
enum QueryAs {
    Password,
}

#[inline]
pub async fn batch(db: &DatabaseConnection) -> Vec<user::Model> {
    User::find().all(db).await.unwrap()
}

pub async fn password(db: &DatabaseConnection, id: i64) -> Option<String> {
    User::find_by_id(id)
        .column_as(user::Column::Password, QueryAs::Password)
        .into_values::<String, QueryAs>()
        .one(db)
        .await
        .unwrap()
}

pub async fn insert(db: &DatabaseConnection, id: i64, nickname: String, password: String) {
    use ActiveValue::*;

    user::ActiveModel {
        id: Set(id),
        nickname: Set(nickname),
        password: Set(password),
    }
    .insert(db)
    .await
    .unwrap();
}
