mod dba;
mod handlers;
mod middlewares;
mod models;
mod router;
mod socket;
mod utils;

use crate::{middlewares::MessageQueue, router::Route};
use dotenv::dotenv;
use sea_orm::Database;
use std::{env, process::ExitCode};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> ExitCode {
    // initiate log instance
    tracing_subscriber::fmt().init();

    // set all the environment variables
    if dotenv().is_err() {
        tracing::error!("No `.env` in current directory or parents");
        return ExitCode::FAILURE;
    }

    // connect database
    let Ok(db_url) = env::var("DATABASE_URL") else {
        tracing::error!("environment variable not found: `{}`", "DATABASE_URL");
        return ExitCode::FAILURE;
    };

    let server = TcpListener::bind("localhost:8080").await.unwrap();
    tracing::info!(ip = "localhost", port = "8080", "server started");

    let db = Database::connect(db_url).await.unwrap();
    let mq = MessageQueue::new(db.clone());
    let route = Route::new(db, mq);

    while let Ok((socket, addr)) = server.accept().await {
        tracing::info!("new client from {addr}");

        let route = route.clone();
        tokio::spawn(async move {
            route.call(socket).await;
        });
    }

    ExitCode::SUCCESS
}
