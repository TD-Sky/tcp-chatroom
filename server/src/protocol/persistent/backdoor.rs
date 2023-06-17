//! 服务任务的后门。在 `serve_connection` 中，
//! 通过取得“后门”的发送端，可将socket发送到“后门”的接收端。

use std::future::Future;
use std::pin::Pin;

use tokio::net::TcpStream;
use tokio::sync::oneshot;

#[derive(Debug)]
pub struct Backdoor {
    sender: Option<oneshot::Sender<TcpStream>>,
    receiver: oneshot::Receiver<TcpStream>,
}

impl Backdoor {
    #[inline]
    pub fn new() -> Self {
        let (sender, receiver) = oneshot::channel();

        Self {
            sender: Some(sender),
            receiver,
        }
    }

    #[inline]
    pub fn take_sender(&mut self) -> oneshot::Sender<TcpStream> {
        self.sender.take().unwrap()
    }
}

impl Future for Backdoor {
    type Output = TcpStream;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.receiver)
            .poll(cx)
            .map(|res| res.unwrap())
    }
}
