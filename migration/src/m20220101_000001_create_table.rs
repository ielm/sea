use sea_orm::{DatabaseBackend, Statement};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let backend = db.get_database_backend();

        if backend == DatabaseBackend::Postgres {
            db.execute(Statement::from_string(
                DatabaseBackend::Postgres,
                "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";",
            ))
            .await?;
        }
        db.execute(Statement::from_string(
            DatabaseBackend::Postgres,
            "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";",
        ))
        .await?;

        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::ExternalId)
                            .uuid()
                            .not_null()
                            .default(uuid::Uuid::new_v4().to_string().as_str()),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null())
                    .col(ColumnDef::new(User::Email).string().not_null())
                    .col(ColumnDef::new(User::IsActive).boolean().not_null())
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("NOW()"),
                    )
                    .col(
                        ColumnDef::new(User::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default("NOW()"),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        db.execute_unprepared("DROP EXTENSION IF EXISTS 'uuid-ossp';")
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    ExternalId,
    Username,
    Email,
    IsActive,
    CreatedAt,
    UpdatedAt,
}
