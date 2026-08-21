use sea_orm_migration::prelude::*;

/// Baseline no-op migration.
///
/// The existing schema lives in `pharmacy_bd/` (DDL.sql + schemas.sql) and is
/// provisioned externally, so this first migration intentionally performs no
/// changes. It exists to establish the `seaql_migrations` bookkeeping table.
///
/// ⚠️ Real schema changes must be added as NEW migration files
/// (`mYYYYMMDD_HHMMSS_description.rs`) registered in `lib.rs`, never by
/// editing an already-applied migration.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No-op: baseline for externally-managed schema.
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // No-op: nothing to roll back.
        Ok(())
    }
}
