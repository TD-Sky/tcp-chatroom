use crate::protocol::short::{Request, Response};

pub fn ping(req: Request) -> Response {
    Response::from("pong")
}
