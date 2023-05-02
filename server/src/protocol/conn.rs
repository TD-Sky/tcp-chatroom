use super::{Request, Response};
use crate::io;
use std::future::Future;
use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpStream;

pub async fn serve_connection<F, R>(mut socket: TcpStream, service_fn: F) -> std::io::Result<()>
where
    F: FnOnce(Request) -> R,
    R: Future<Output = Response>,
{
    let request = io::read_request(BufReader::new(&mut socket)).await.unwrap();
    let response = service_fn(request).await;
    io::write_response(BufWriter::new(&mut socket), response)
        .await
        .unwrap();

    Ok(())
}
