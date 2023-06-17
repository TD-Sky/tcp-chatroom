use std::io;

use tokio::net::TcpListener;
use tracing::info;

use crate::connection::serve_short;
use crate::service::Router;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub async fn new(addr: &str, port: u16) -> io::Result<Self> {
        info!(ip = addr, port = port, "server started");
        Ok(Self {
            listener: TcpListener::bind((addr, port)).await?,
        })
    }

    /// 监听通信，抽取通信权柄交给路由器处理
    pub async fn run(&self) {
        let router = Router;

        while let Ok((socket, addr)) = self.listener.accept().await {
            info!("connection came from {addr}");

            let router = router.clone();
            tokio::spawn(async move {
                serve_short(socket, |req| router.short_call(req))
                    .await
                    .unwrap();
            });
        }
    }
}
