use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicMessage {
    /// 发言人
    pub uid: String,

    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateMessage {
    /// 发言人 或 私信对象
    pub uid: i64,

    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMessage {
    /// 发言人 或 收信人
    pub uid: i64,

    /// 群号
    pub gid: i32,

    pub content: String,
}
