use std::sync::atomic;
use std::sync::atomic::AtomicU32;

use tokio::sync::mpsc;
use tracing::debug;

use crate::connection::serve_persistent;
use crate::middlewares::mq;
use crate::models::{Echo, PrivateMsg};
use crate::protocol::short::Request;
use crate::protocol::short::{response::Status, Response};
use crate::service::Router;

static UID: AtomicU32 = AtomicU32::new(0);

pub async fn connect(mut req: Request) -> Response {
    let backdoor = req.backdoor.take().unwrap();

    tokio::spawn(async {
        let socket = backdoor.await;

        let (sender, receiver) = mpsc::channel(32);
        let uid = UID.fetch_add(1, atomic::Ordering::SeqCst);
        mq().insert_user(uid, sender);

        serve_persistent(socket, receiver, |in_msg| Router.persist_call(uid, in_msg))
            .await
            .unwrap();
    });

    Response::from(Status::Ok)
}

pub async fn privately_msg(uid: u32, mut pmsg: PrivateMsg) -> Echo {
    debug!(
        from_uid = uid,
        to_uid = pmsg.uid,
        content = &pmsg.content,
        "sending private message"
    );

    mq().push_private(uid, &mut pmsg).await;
    pmsg.into()
}
