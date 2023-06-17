mod socket;
mod middlewares;
mod models;
mod protocol;
mod service;
mod connection;

mod server;
use server::Server;
use tracing::Level;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let server = Server::new("127.0.0.1", 8080).await.unwrap();
    server.run().await;
}
