use crate::{
    handlers::{self, guard},
    middlewares::{auth, MessageQueue},
    models::MaybeNewUser,
    socket,
};
use entity::group;
use protocol::short::{request, response::Status, Response};
use sea_orm::DatabaseConnection;
use tokio::{
    io::{BufReader, BufWriter},
    net::TcpStream,
};

#[derive(Clone)]
pub struct Route {
    db: DatabaseConnection,
    mq: MessageQueue,
}

impl Route {
    pub fn new(db: DatabaseConnection, mq: MessageQueue) -> Self {
        Self { db, mq }
    }

    pub async fn call(&self, mut socket: TcpStream) {
        let Self { db, mq } = self;

        let req = socket::read_request(BufReader::new(&mut socket)).await;

        if let request::Method::Login = req.method() {
            let user: MaybeNewUser = rmp_serde::from_slice(req.data().unwrap()).unwrap();

            let resp = if user.id.is_some() {
                guard::login(user, &self.db).await
            } else {
                guard::register(user, &self.db).await
            };

            socket::write_response(BufWriter::new(&mut socket), resp).await;
            return;
        }

        let uid = match req
            .token()
            .ok_or_else(|| Response::from(Status::Unauthorized))
            .and_then(auth)
        {
            Ok(uid) => uid,
            Err(resp) => {
                socket::write_response(BufWriter::new(&mut socket), resp).await;
                return;
            }
        };

        let resp = {
            use request::Method::*;

            match req.method() {
                Persistence => {
                    handlers::chat::routine(socket, uid, mq.clone()).await;
                    return;
                }

                Groups => handlers::group::batch(db).await,

                CreateGroup => {
                    let group: group::InsertModel =
                        rmp_serde::from_slice(req.data().unwrap()).unwrap();
                    handlers::group::create(uid, group, db).await
                }

                JoinGroup => {
                    let gid: i32 = rmp_serde::from_slice(req.data().unwrap()).unwrap();
                    handlers::group::join(gid, uid, db).await
                }

                _ => unreachable!(),
            }
        };

        socket::write_response(BufWriter::new(&mut socket), resp).await;
    }
}
