mod io;
mod protocol;
mod server;
mod service;

use self::server::Server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let server = Server::new("127.0.0.1", 8080).await.unwrap();
    server.run().await;
}
