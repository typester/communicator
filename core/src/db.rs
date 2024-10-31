use std::{collections::HashMap, fs::File, path::Path, sync::Arc};

use sqlx::{migrate::Migrate, Connection, SqliteConnection, SqlitePool};

use crate::error::CommunicatorError;

#[derive(Debug)]
pub struct Db {
    database_url: String,
    pool: SqlitePool,
}

impl Db {
    pub fn new(database_url: String) -> anyhow::Result<Arc<Self>> {
        let pool = SqlitePool::connect_lazy(&database_url)?;
        Ok(Arc::new(Self { database_url, pool }))
    }

    pub async fn migration_available(&self) -> Result<bool, CommunicatorError> {
        let migrator = sqlx::migrate!();
        // Seems like the SqlitePool doesn't implement Migrate trait
        let mut conn = SqliteConnection::connect(&self.database_url).await?;

        conn.ensure_migrations_table().await?;

        let applied_migrations: HashMap<_, _> = conn
            .list_applied_migrations()
            .await?
            .into_iter()
            .map(|m| (m.version, m))
            .collect();

        let has_mismatched_migration = migrator
            .iter()
            .filter(|m| !m.migration_type.is_down_migration())
            .find(|m| {
                if let Some(applied) = applied_migrations.get(&m.version) {
                    m.checksum != applied.checksum
                } else {
                    false
                }
            });
        if has_mismatched_migration.is_some() {
            return Err(CommunicatorError::MigrateError(
                "mismatched migration is found".into(),
            ));
        }

        let has_unapplied_migration = match migrator
            .iter()
            .filter(|m| !m.migration_type.is_down_migration())
            .find(|m| !applied_migrations.contains_key(&m.version))
        {
            Some(_) => true,
            None => false,
        };

        Ok(has_unapplied_migration)
    }

    pub async fn do_migration(&self) -> anyhow::Result<()> {
        let pool = self.pool.clone();
        let migrator = sqlx::migrate!();
        Ok(migrator.run(&pool).await?)
    }

    pub fn reset(&self) -> anyhow::Result<()> {
        let path = Path::new(&self.database_url["sqlite://".len()..]);
        let f = File::create(path)?;
        f.set_len(0)?;
        Ok(())
    }
}
