use std::future::Future;
use std::io;

use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpStream;
use tokio::sync::mpsc;
use tracing::info;

use crate::protocol::persistent::Message;
use crate::protocol::short::{Request, Response};
use crate::socket;

pub async fn serve_short<F, R>(mut socket: TcpStream, service_fn: F) -> io::Result<()>
where
    F: FnOnce(Request) -> R,
    R: Future<Output = Response>,
{
    info!("serving short connection");

    let mut request = socket::short::read_request(BufReader::new(&mut socket))
        .await
        .unwrap();

    let sender = request
        .backdoor
        .as_mut()
        .map(|backdoor| backdoor.take_sender());

    let response = service_fn(request).await;
    socket::short::write_response(BufWriter::new(&mut socket), response)
        .await
        .unwrap();

    if let Some(sender) = sender {
        sender.send(socket).unwrap();
    }

    Ok(())
}

pub async fn serve_persistent<F, R>(
    mut socket: TcpStream,
    mut receiver: mpsc::Receiver<Message>,
    service_fn: F,
) -> io::Result<()>
where
    F: FnOnce(Message) -> R + Copy,
    R: Future<Output = Option<Message>>,
{
    let addr = socket.peer_addr().unwrap();

    loop {
        tokio::select! {
            internal_msg = receiver.recv() => {
                info!(addr = ?addr, "received message from internal");
                let Some(internal_msg) = internal_msg else {
                    break;
                };

                info!(addr = ?addr, "writing internal message");
                socket::persistent::write_message(BufWriter::new(&mut socket), internal_msg)
                    .await
                    .unwrap();
            },

            incoming_msg = socket::persistent::read_message(BufReader::new(&mut socket)) => {
                let Ok(incoming_msg) = incoming_msg else {
                    info!(addr = ?addr, "user leaves");
                    break;
                };
                info!(addr = ?addr, "read a message");

                if let Some(outcoming_msg) = service_fn(incoming_msg).await {
                    info!(addr = ?addr, "writing a message");
                    socket::persistent::write_message(BufWriter::new(&mut socket), outcoming_msg)
                        .await
                        .unwrap();
                }
            },
        };
    }

    Ok(())
}
