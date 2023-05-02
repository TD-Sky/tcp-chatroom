use crate::protocol::{Request, Response};

pub fn ping(req: Request) -> Response {
    Response::from("pong")
}
