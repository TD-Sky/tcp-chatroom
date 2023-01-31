use crate::dba;
use entity::group;
use protocol::short::{response::Status, Response};
use sea_orm::DatabaseConnection;

pub async fn user_groups(uid: i64, db: &DatabaseConnection) -> Response {
    let groups = dba::group_user::user_groups(db, uid).await;
    Response::new(groups)
}

pub async fn batch(db: &DatabaseConnection) -> Response {
    let groups = dba::group::batch(db).await;
    Response::new(groups)
}

pub async fn create(owner_id: i64, group: group::InsertModel, db: &DatabaseConnection) -> Response {
    match dba::group::insert(db, owner_id, group).await {
        Ok(group) => Response::new(group),
        Err(_) => Response::from(Status::GroupExisted),
    }
}

pub async fn join(gid: i32, uid: i64, db: &DatabaseConnection) -> Response {
    let joined_group = dba::group_user::insert(db, gid, uid).await.unwrap();
    Response::new(joined_group)
}
