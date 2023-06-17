use std::borrow::Cow;

use crate::protocol::Body;

#[derive(Debug)]
pub struct Request {
    method: Cow<'static, str>,
    body: Body,
}

impl Request {
    #[inline]
    pub fn new(method: Cow<'static, str>, body: Body) -> Self {
        Self { method, body }
    }

    #[inline]
    pub fn method(&self) -> &str {
        self.method.as_ref()
    }

    #[inline]
    pub fn body(&self) -> &Body {
        &self.body
    }
}
