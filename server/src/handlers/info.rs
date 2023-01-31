use std::collections::HashMap;

use crate::{dba, middlewares::MessageQueue};
use protocol::short::Response;
use sea_orm::DatabaseConnection;

pub async fn id_name_map(uid: i64, mq: &MessageQueue, db: &DatabaseConnection) -> Response {
    let user_groups = dba::group_user::user_groups(db, uid).await;

    let mut all_users = dba::user::batch(db).await;
    let mut online_users_id = mq.uids().await;
    all_users.sort_by_key(|user| user.id);
    online_users_id.sort();

    let mut map = HashMap::with_capacity(user_groups.len() + online_users_id.len());
    for group in user_groups {
        map.insert(group.id as i64, group.name);
    }
    let mut i = 0;
    let len = online_users_id.len();
    for user in all_users {
        if user.id == online_users_id[i] {
            map.insert(user.id, user.nickname);
            i += 1;
        }

        if i > len {
            break;
        }
    }

    Response::new(map)
}
