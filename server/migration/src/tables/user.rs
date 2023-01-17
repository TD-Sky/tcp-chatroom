use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum User {
    Table,

    /// UID，使用雪花算法生成的 i64 主键
    Id,

    /// 昵称，用户间允许重用
    Nickname,

    /// 密码，使用 SHA256 加盐哈希
    Password,
}

impl User {
    pub fn table() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(ColumnDef::new(Self::Id).big_integer().primary_key())
            .col(ColumnDef::new(Self::Nickname).string().not_null())
            .col(ColumnDef::new(Self::Password).char_len(64).not_null())
            .to_owned()
    }

    pub fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}
