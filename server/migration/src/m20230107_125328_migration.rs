use crate::tables::{Group, GroupUser, User};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.create_table(User::table()).await?;
        manager.create_table(Group::table()).await?;
        manager.create_table(GroupUser::table()).await?;

        manager.create_foreign_key(Group::own_by_user()).await?;
        manager.create_foreign_key(GroupUser::the_group()).await?;
        manager.create_foreign_key(GroupUser::the_member()).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(User::drop()).await?;
        manager.drop_table(Group::drop()).await?;
        manager.drop_table(GroupUser::drop()).await?;

        Ok(())
    }
}
