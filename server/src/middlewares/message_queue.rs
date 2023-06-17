use std::mem;

use dashmap::DashMap;
use tokio::sync::mpsc;

use crate::models::PrivateMsg;
use crate::protocol::persistent::*;
use crate::protocol::Body;

#[derive(Debug, Default)]
pub struct MessageQueue(DashMap<u32, mpsc::Sender<Message>>);

impl MessageQueue {
    #[inline]
    pub fn insert_user(&self, uid: u32, sender: mpsc::Sender<Message>) {
        self.0.insert(uid, sender);
    }

    pub async fn push_private(&self, from_uid: u32, pmsg: &mut PrivateMsg) {
        let to_uid = mem::replace(&mut pmsg.uid, from_uid);

        self.0
            .get_mut(&to_uid)
            .unwrap()
            .send(Message::new(Method::Private, Body::serialize(pmsg)))
            .await
            .unwrap();
    }
}
