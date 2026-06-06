use sqlx::{mysql::MySqlPoolOptions, Executor, MySqlPool};
use std::env;

#[derive(Clone)]
pub struct Database {
    pub(crate) pool: Option<MySqlPool>,
    pub(crate) db_url: &str,
}

impl Database {
    pub async fn new(db_url: &str) -> Self {
        Database { None, db_url }
    }

    pub async fn connect(&mut self) -> Result<(), sqlx::Error> {
        let pool = MySqlPoolOptions::new().connect(self.db_url).await?;
        self.pool = Some(pool);
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        if let Some(pool) = &self.pool {
            pool.close().await;
            self.pool = None;
        }
    }

    pub async fn execute_query(&self, query: &str) -> Result<(), sqlx::Error> {
        self.pool.execute(query).await?;
        Ok(())
    }
}
