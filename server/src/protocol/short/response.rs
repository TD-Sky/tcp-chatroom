#[derive(Debug)]
pub struct Response {
    status: Status,
    body: Option<Vec<u8>>,
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug)]
pub enum Status {
    Ok,
}

impl Response {
    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }
}

impl Status {
    pub fn code(&self) -> u8 {
        use Status::*;
        match self {
            Ok => 0,
        }
    }
}

impl From<Status> for Response {
    #[inline]
    fn from(status: Status) -> Self {
        Self { status, body: None }
    }
}

impl From<&'static str> for Response {
    #[inline]
    fn from(msg: &'static str) -> Self {
        Self {
            status: Status::Ok,
            body: Some(rmp_serde::to_vec(msg).unwrap()),
        }
    }
}
