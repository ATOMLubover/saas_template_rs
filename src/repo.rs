use std::ops::Deref;

use sea_orm::{Database, DatabaseConnection};

#[derive(Debug)]
pub struct Repository {
    database: DatabaseConnection,
}

impl Repository {
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|err| anyhow::anyhow!("Error when acquiring DATABASE_URL: {}", err))?;

        let database = Database::connect(database_url)
            .await
            .map_err(|err| anyhow::anyhow!("Error when connecting to Database: {}", err))?;

        Ok(Self { database })
    }
}

impl Deref for Repository {
    type Target = DatabaseConnection;

    fn deref(&self) -> &Self::Target {
        &self.database
    }
}
