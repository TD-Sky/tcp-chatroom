use crate::{
    dba,
    models::MaybeNewUser,
    utils::{codec, hasher, uid_bucket},
};
use protocol::short::{response::Status, Response};
use sea_orm::DatabaseConnection;

pub async fn login(user: MaybeNewUser, db: &DatabaseConnection) -> Response {
    let uid = user.id.unwrap();
    let correct_pswd = dba::user::password(db, uid).await;

    match correct_pswd {
        None => Response::from(Status::UserNotFound),
        Some(correct_pswd) => {
            let pswd = hasher::hash(&user.password);

            if pswd == correct_pswd {
                let token = codec::gen(uid).unwrap();
                Response::new(token)
            } else {
                Response::from(Status::WrongPassword)
            }
        }
    }
}

pub async fn register(user: MaybeNewUser, db: &DatabaseConnection) -> Response {
    let uid = uid_bucket::pop().await;
    let hashed_pswd = hasher::hash(&user.password);

    dba::user::insert(db, uid, user.nickname, hashed_pswd).await;

    let token = codec::gen(uid).unwrap();
    Response::new(token)
}
