use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct LoginUser {
    pub id: i64,
    pub nickname: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct RegisterUser {
    pub nickname: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub owner_id: i64,
}
