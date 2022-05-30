use migration::Migrator;
use migration::MigratorTrait;
use sea_orm::Database;
use sea_orm::DbBackend;
use sea_orm::DbConn;
use sea_orm::DbErr;
use sea_orm::Schema;

pub async fn init_database() -> Result<DbConn, DbErr> {
    let db = Database::connect("sqlite::memory:").await?;
    Migrator::up(&db, None).await?;
    setup_schema().await;
    Ok(db)
}

async fn setup_schema() {
    Schema::new(DbBackend::Sqlite);
}
