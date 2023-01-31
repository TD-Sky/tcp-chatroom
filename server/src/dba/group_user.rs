use entity::{
    group, group_user,
    prelude::{Group, GroupUser},
};
use sea_orm::{prelude::*, IntoActiveModel, QuerySelect};

#[derive(Debug, Clone, Copy, DeriveColumn, EnumIter)]
enum QueryAs {
    Uid,
}

pub async fn user_groups(db: &DatabaseConnection, uid: i64) -> Vec<group::Model> {
    GroupUser::find()
        .left_join(Group)
        .select_only()
        .column(group::Column::Id)
        .column(group::Column::Name)
        .column(group::Column::OwnerId)
        .filter(group_user::Column::Uid.eq(uid))
        .into_model::<group::Model>()
        .all(db)
        .await
        .unwrap()
}

pub async fn group_users_id(db: &DatabaseConnection, gid: i32) -> Vec<i64> {
    GroupUser::find()
        .select_only()
        .filter(group_user::Column::Gid.eq(gid))
        .column_as(group_user::Column::Uid, QueryAs::Uid)
        .into_values::<i64, QueryAs>()
        .all(db)
        .await
        .unwrap()
}

pub async fn insert(db: &DatabaseConnection, gid: i32, uid: i64) -> Option<group::Model> {
    let _ = group_user::InsertModel { uid, gid }
        .into_active_model()
        .insert(db)
        .await
        .unwrap();

    Group::find_by_id(gid).one(db).await.unwrap()
}
