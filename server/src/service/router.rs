use crate::protocol::request::Method;
use crate::protocol::Request;
use crate::protocol::Response;

use super::handlers;

// NOTE: Router用来保存数据库连接池及其它权柄
#[derive(Debug, Clone)]
pub struct Router;

impl Router {
    pub async fn call(&self, req: Request) -> Response {
        match req.method() {
            Method::Ping => handlers::ping(req),
        }
    }
}
