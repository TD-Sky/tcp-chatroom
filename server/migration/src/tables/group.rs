use super::User;
use sea_orm_migration::prelude::*;

#[derive(Iden)]
pub enum Group {
    Table,

    /// GID，自增的 i32 主键
    Id,

    /// 群名称，用户间可重用，用户名下不可重用
    Name,

    /// 群主的UID
    OwnerId,
}

impl Group {
    pub fn table() -> TableCreateStatement {
        Table::create()
            .table(Self::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(Self::Id)
                    .integer()
                    .auto_increment()
                    .primary_key(),
            )
            .col(ColumnDef::new(Self::Name).string().not_null())
            .col(ColumnDef::new(Self::OwnerId).big_integer().not_null())
            .to_owned()
    }

    pub fn own_by_user() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .name("FK_group_user")
            .from(Group::Table, Group::OwnerId)
            .to(User::Table, User::Id)
            .to_owned()
    }

    pub fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}
