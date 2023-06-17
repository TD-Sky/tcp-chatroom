use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrivateMsg {
    pub uid: u32,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Echo {
    pub id: u32,
    pub context: Context,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Context {
    Private,
    Group,
}

impl From<PrivateMsg> for Echo {
    #[inline]
    fn from(pmsg: PrivateMsg) -> Self {
        Self {
            id: pmsg.uid,
            context: Context::Private,
            content: pmsg.content,
        }
    }
}
