use crate::{middlewares::MessageQueue, models::Message, socket};
use protocol::{
    models::{GroupMessage, PrivateMessage, PublicMessage},
    persistent::{Horz, Method},
    short::{response::Status, Response},
};
use tokio::{io::BufStream, net::TcpStream};

pub async fn routine(socket: TcpStream, uid: i64, mq: MessageQueue) {
    let mut socket = BufStream::new(socket);
    let mut receiver = mq.insert_user(uid).await;
    socket::write_response(&mut socket, Response::from(Status::Ok)).await;

    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(msg) = receiver.recv() => {
                    let outcoming_horz = match msg {
                        Message::Public(msg) => {
                            let data = rmp_serde::to_vec(&msg).unwrap();
                            Horz::new(Method::PublicMessage, data)
                        }

                        Message::Private(msg) => {
                            let data = rmp_serde::to_vec(&msg).unwrap();
                            Horz::new(Method::PrivateMessage, data)
                        }

                        Message::Group(msg) => {
                            let data = rmp_serde::to_vec(&msg).unwrap();
                            Horz::new(Method::GroupMessage, data)
                        }
                    };

                    socket::write_horz(&mut socket, outcoming_horz).await;
                },

                mut incoming_horz = socket::read_horz(&mut socket) => {
                    match incoming_horz.method() {
                        Method::Ping => {
                            let outcoming_horz = Horz::from(Method::Pong);
                            socket::write_horz(&mut socket, outcoming_horz).await;
                        }

                        Method::Pong => {
                            let outcoming_horz = Horz::from(Method::Ping);
                            socket::write_horz(&mut socket, outcoming_horz).await;
                        }

                        Method::Close => {
                            mq.remove_user(uid).await;
                            break;
                        }

                        Method::PublicMessage =>{
                            let msg: PublicMessage = rmp_serde::from_slice(
                                incoming_horz.data().unwrap()
                            ).unwrap();
                            mq.push(uid, Message::Public(msg)).await;
                        }

                        Method::PrivateMessage => {
                            let pmsg: PrivateMessage = rmp_serde::from_slice(
                                incoming_horz.data().unwrap()
                            ).unwrap();
                            mq.push(uid, Message::Private(pmsg)).await;
                            incoming_horz.set_method(Method::Echo);
                            let outcoming_horz = incoming_horz;
                            socket::write_horz(&mut socket, outcoming_horz).await;
                        }

                        Method::GroupMessage => {
                            let gmsg: GroupMessage  = rmp_serde::from_slice(
                                incoming_horz.data().unwrap()
                            ).unwrap();
                            mq.push(uid, Message::Group(gmsg)).await;
                        }

                        _ => unreachable!()
                    }
                },
            }
        }
    });
}
