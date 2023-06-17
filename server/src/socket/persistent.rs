use std::io;

use tokio::io::AsyncWriteExt;
use tokio::io::{AsyncBufReadExt, AsyncReadExt};

use crate::protocol::persistent::Message;
use crate::protocol::persistent::Method;
use crate::protocol::Body;

pub async fn read_message(mut reader: impl AsyncBufReadExt + Unpin) -> io::Result<Message> {
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

    Ok(Message::new(method, body))
}

pub async fn write_message(mut writer: impl AsyncWriteExt + Unpin, msg: Message) -> io::Result<()> {
    let method = msg.method().to_string();
    writer.write_u64(method.len() as u64).await?;
    writer.write_all(method.as_bytes()).await?;

    if let Some(body) = msg.body().try_as_bytes() {
        writer.write_u16(body.len() as u16).await?;
        writer.write_all(body).await?;
    } else {
        writer.write_u16(0).await?;
    }

    writer.flush().await
}
