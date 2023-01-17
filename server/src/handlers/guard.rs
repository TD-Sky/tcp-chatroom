use crate::{
    models::MaybeNewUser,
    utils::{codec, hasher, uid_bucket},
};
use entity::{prelude::User, user};
use protocol::short::{response::Status, Response};
use sea_orm::{
    ActiveModelTrait, ActiveValue, DatabaseConnection, DeriveColumn, EntityTrait, EnumIter,
    QuerySelect,
};

#[derive(Debug, Clone, Copy, DeriveColumn, EnumIter)]
enum QueryAs {
    Password,
}

pub async fn login(user: MaybeNewUser, db: &DatabaseConnection) -> Response {
    let uid = user.id.unwrap();
    let correct_pswd = User::find_by_id(uid)
        .column_as(user::Column::Password, QueryAs::Password)
        .into_values::<String, QueryAs>()
        .one(db)
        .await
        .unwrap();

    match correct_pswd {
        None => Response::from(Status::UserNotFound),
        Some(correct_pswd) => {
            let pswd = hasher::hash(&user.password);

            if pswd == correct_pswd {
                let token = codec::gen(uid).unwrap();
                let data = rmp_serde::to_vec(&token).unwrap();
                Response::new(Status::Ok, data)
            } else {
                Response::from(Status::WrongPassword)
            }
        }
    }
}

pub async fn register(user: MaybeNewUser, db: &DatabaseConnection) -> Response {
    let uid = uid_bucket::pop().await;
    let hashed_pswd = hasher::hash(&user.password);

    let insert_model = user::ActiveModel {
        id: ActiveValue::Set(uid),
        nickname: ActiveValue::Set(user.nickname),
        password: ActiveValue::Set(hashed_pswd),
    };
    insert_model.insert(db).await.unwrap();

    let token = codec::gen(uid).unwrap();
    let data = rmp_serde::to_vec(&token).unwrap();
    Response::new(Status::Ok, data)
}
