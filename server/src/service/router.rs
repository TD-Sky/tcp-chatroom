use tracing::info;

use crate::protocol::persistent::Message;
use crate::protocol::short::{Request, Response};
use crate::protocol::Body;
use crate::service::handlers::persistent;
use crate::service::handlers::short;

#[derive(Debug, Clone)]
pub struct Router;

impl Router {
    pub async fn short_call(&self, req: Request) -> Response {
        use crate::protocol::short::Method;

        match req.method() {
            Method::Ping => short::ping(req),
            Method::Persistent => persistent::connect(req).await,
        }
    }

    pub async fn persist_call(&self, uid: u32, incoming_msg: Message) -> Option<Message> {
        use crate::protocol::persistent::Method;

        let echo = match incoming_msg.method() {
            Method::Private => Some(
                persistent::privately_msg(uid, incoming_msg.body().deserialize().unwrap()).await,
            ),
            _ => unimplemented!(),
        };

        echo.map(|e| Message::new(Method::Echo, Body::serialize(&e)))
    }
}
