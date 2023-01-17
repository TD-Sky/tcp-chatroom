use super::{Group, User};
use sea_orm_migration::prelude::*;

/// 群成员
#[derive(Iden)]
pub enum GroupUser {
    Table,

    /// 自增的 i32 主键
    Id,

    /// 群组ID，外键
    Gid,

    /// 用户ID，外键
    Uid,
}

impl GroupUser {
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
            .col(ColumnDef::new(Self::Gid).integer().not_null())
            .col(ColumnDef::new(Self::Uid).big_integer().not_null())
            .to_owned()
    }

    pub fn the_group() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .name("FK_group_user_group")
            .from(GroupUser::Table, GroupUser::Gid)
            .to(Group::Table, Group::Id)
            .to_owned()
    }

    pub fn the_member() -> ForeignKeyCreateStatement {
        ForeignKey::create()
            .name("FK_group_user_user")
            .from(GroupUser::Table, GroupUser::Uid)
            .to(User::Table, User::Id)
            .to_owned()
    }

    pub fn drop() -> TableDropStatement {
        Table::drop().table(Self::Table).to_owned()
    }
}
