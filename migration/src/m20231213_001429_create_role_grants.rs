use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoleGrant::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoleGrant::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RoleGrant::RoleKey).integer().not_null())
                    .col(ColumnDef::new(RoleGrant::UserId).integer().not_null())
                    .col(ColumnDef::new(RoleGrant::ResourceTable).string().not_null())
                    .col(ColumnDef::new(RoleGrant::ResourceId).string().not_null())
                    .col(
                        ColumnDef::new(RoleGrant::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default("NOW()"),
                    )
                    .col(
                        ColumnDef::new(RoleGrant::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default("NOW()"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RoleGrant::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum RoleGrant {
    Table,
    Id,
    RoleKey,
    UserId,
    ResourceTable,
    ResourceId,
    CreatedAt,
    UpdatedAt,
}
