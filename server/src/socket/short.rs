use std::io;

use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

use crate::protocol::short::Response;
use crate::protocol::short::{Method, Request};
use crate::protocol::Body;

pub async fn read_request(mut reader: impl AsyncBufReadExt + Unpin) -> io::Result<Request> {
    let mlen = reader.read_u64().await?;
    let mut method = String::with_capacity(mlen as usize);
    (&mut reader).take(mlen).read_to_string(&mut method).await?;
    let method: Method = method.parse().unwrap();

    let blen = reader.read_u16().await?;
    let body = if blen != 0 {
        let mut body = Vec::with_capacity(blen as usize);
        (&mut reader)
            .take(blen as u64)
            .read_to_end(&mut body)
            .await?;
        Body::from(body)
    } else {
        Body::empty()
    };

    Ok(Request::new(method, body))
}

pub async fn write_response(
    mut writer: impl AsyncWriteExt + Unpin,
    resp: Response,
) -> io::Result<()> {
    writer.write_u8(resp.status().code()).await?;

    if let Some(body) = resp.body() {
        writer.write_u64(body.len() as u64).await?;
        writer.write_all(body).await?;
    } else {
        writer.write_u64(0).await?;
    }

    writer.flush().await
}
