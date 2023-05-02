use crate::protocol;
use crate::protocol::Request;
use crate::protocol::Response;
use protocol::request;

use std::io;
use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

pub async fn read_request(mut reader: impl AsyncBufReadExt + Unpin) -> io::Result<Request> {
    let mlen = reader.read_u64_le().await.unwrap();
    let mut method = String::with_capacity(mlen as usize);
    (&mut reader)
        .take(mlen)
        .read_to_string(&mut method)
        .await
        .unwrap();
    let method: request::Method = method.parse().unwrap();

    let blen = reader.read_u16().await.unwrap();
    let body = if blen != 0 {
        let mut body = Vec::with_capacity(blen as usize);
        (&mut reader)
            .take(blen as u64)
            .read_to_end(&mut body)
            .await
            .unwrap();
        Some(body)
    } else {
        None
    };

    Ok(Request::new(method, body))
}

pub async fn write_response(
    mut writer: impl AsyncWriteExt + Unpin,
    resp: Response,
) -> io::Result<()> {
    writer.write_u8(resp.status().code()).await.unwrap();

    if let Some(body) = resp.body() {
        writer.write_u64_le(body.len() as u64).await.unwrap();
        writer.write_all(body).await.unwrap();
    } else {
        writer.write_u64_le(0).await.unwrap();
    }

    writer.flush().await
}
