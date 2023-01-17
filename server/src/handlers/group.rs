use entity::{group, group_user, prelude::Group};
use protocol::short::{response::Status, Response};
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, IntoActiveModel};

pub async fn batch(db: &DatabaseConnection) -> Response {
    let groups = Group::find().all(db).await.unwrap();
    let data = rmp_serde::to_vec(&groups).unwrap();
    Response::new(Status::Ok, data)
}

pub async fn create(owner_id: i64, group: group::InsertModel, db: &DatabaseConnection) -> Response {
    let insert_act_model = group::ActiveModel {
        id: ActiveValue::NotSet,
        name: ActiveValue::Set(group.name),
        owner_id: ActiveValue::Set(owner_id),
    };

    match insert_act_model.insert(db).await {
        Ok(group) => {
            let data = rmp_serde::to_vec(&group).unwrap();
            Response::new(Status::Ok, data)
        }
        Err(_) => Response::from(Status::GroupExisted),
    }
}

pub async fn join(gid: i32, uid: i64, db: &DatabaseConnection) -> Response {
    let insert_model = group_user::InsertModel { uid, gid };
    let _ = insert_model.into_active_model().insert(db).await;

    let group = Group::find_by_id(gid).one(db).await.unwrap().unwrap();
    let data = rmp_serde::to_vec(&group).unwrap();
    Response::new(Status::Ok, data)
}
