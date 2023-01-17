use crate::models::Message;
use entity::{group_user, prelude::GroupUser};
use sea_orm::{
    ColumnTrait, DatabaseConnection, DeriveColumn, EntityTrait, EnumIter, QueryFilter, QuerySelect,
};
use std::{collections::HashMap, mem, sync::Arc};
use tokio::sync::{mpsc, RwLock};

#[derive(Debug, Clone, Copy, DeriveColumn, EnumIter)]
enum QueryAs {
    Uid,
}

#[derive(Clone)]
pub struct MessageQueue {
    inner: Arc<RwLock<HashMap<i64, mpsc::Sender<Message>>>>,
    db: DatabaseConnection,
}

impl MessageQueue {
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            db,
        }
    }

    pub async fn insert_user(&self, uid: i64) -> mpsc::Receiver<Message> {
        let (sender, receiver) = mpsc::channel(32);
        let mut uid2sender = self.inner.write().await;
        let _ = uid2sender.insert(uid, sender);
        receiver
    }

    pub async fn remove_user(&self, uid: i64) {
        let mut uid2sender = self.inner.write().await;
        uid2sender.remove(&uid).unwrap();
    }

    pub async fn push(&self, from_uid: i64, msg: Message) {
        match msg {
            Message::Public(_) => {
                let uid2sender = self.inner.read().await;
                for sender in uid2sender.values() {
                    sender.send(msg.clone()).await.unwrap();
                }
            }

            Message::Private(mut pmsg) => {
                let to_uid = mem::replace(&mut pmsg.uid, from_uid);
                let uid2sender = self.inner.read().await;
                let sender = uid2sender.get(&to_uid).unwrap();
                sender.send(Message::Private(pmsg)).await.unwrap();
            }

            Message::Group(gmsg) => {
                let to_uids = GroupUser::find()
                    .select_only()
                    .filter(group_user::Column::Gid.eq(gmsg.gid))
                    .column_as(group_user::Column::Uid, QueryAs::Uid)
                    .into_values::<i64, QueryAs>()
                    .all(&self.db)
                    .await
                    .unwrap();

                let uid2sender = self.inner.read().await;
                for sender in uid2sender
                    .iter()
                    .filter_map(|(k, v)| to_uids.contains(k).then_some(v))
                {
                    sender.send(Message::Group(gmsg.clone())).await.unwrap();
                }
            }
        }
    }
}
