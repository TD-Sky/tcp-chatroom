use protocol::{
    parser,
    persistent::{self, Horz},
    short::{request, Request, Response},
};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};

pub async fn read_request(mut reader: impl AsyncBufReadExt + Unpin) -> Request {
    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();
    let method: request::Method = line.parse().unwrap();

    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();

    let token = line
        .starts_with("Token")
        .then(|| parser::token(&line).unwrap());

    let length = if token.is_some() {
        let mut line = String::new();
        reader.read_line(&mut line).await.unwrap();
        parser::length(&line).unwrap()
    } else {
        parser::length(&line).unwrap()
    };

    let mut req = Request::from(method);

    if let Some(token) = token {
        req.set_token(token);
    }

    if length != 0 {
        let mut data = Vec::with_capacity(length as usize);
        (&mut reader)
            .take(length)
            .read_to_end(&mut data)
            .await
            .unwrap();
        req.set_data(data);
    }

    req
}

pub async fn write_response(mut writer: impl AsyncWriteExt + Unpin, resp: Response) {
    writer
        .write_all(resp.status().to_string().as_bytes())
        .await
        .unwrap();
    writer.write_u8(b'\n').await.unwrap();

    if let Some(data) = resp.data() {
        writer
            .write_all(format!("Length = {}\n", data.len() as u64).as_bytes())
            .await
            .unwrap();
        writer.write_all(data).await.unwrap();
    } else {
        writer.write_all(b"Length = 0\n").await.unwrap();
    }

    writer.flush().await.unwrap();
}

pub async fn read_horz(mut reader: impl AsyncBufReadExt + Unpin) -> Horz {
    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();
    let method: persistent::Method = line.parse().unwrap();

    let mut line = String::new();
    reader.read_line(&mut line).await.unwrap();
    let length = parser::length(&line).unwrap();

    if length != 0 {
        let mut data = Vec::with_capacity(length as usize);
        (&mut reader)
            .take(length)
            .read_to_end(&mut data)
            .await
            .unwrap();
        Horz::new(method, data)
    } else {
        Horz::from(method)
    }
}

pub async fn write_horz(mut writer: impl AsyncWriteExt + Unpin, horz: Horz) {
    writer
        .write_all(horz.method().to_string().as_bytes())
        .await
        .unwrap();
    writer.write_u8(b'\n').await.unwrap();

    if let Some(data) = horz.data() {
        writer
            .write_all(format!("Length = {}\n", data.len() as u64).as_bytes())
            .await
            .unwrap();
        writer.write_all(data).await.unwrap();
    } else {
        writer.write_all(b"Length = 0\n").await.unwrap();
    }
}
