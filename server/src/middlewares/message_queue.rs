use crate::{dba, models::Message};
use sea_orm::DatabaseConnection;
use std::{collections::HashMap, mem, sync::Arc};
use tokio::sync::{mpsc, RwLock};

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

    pub async fn uids(&self) -> Vec<i64> {
        self.inner.read().await.keys().cloned().collect()
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
                let to_uids = dba::group_user::group_users_id(&self.db, gmsg.gid).await;
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
