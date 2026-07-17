use sqlx::{mysql::MySqlPoolOptions, mysql::MySqlRow, FromRow, MySqlPool};

/// Encapsulates database connection and operations using sqlx for MySQL.
/// This will be useful for our ORM layer to manage database interactions in a clean and efficient way.
/// This struct provides methods to connect to the database, execute queries, and disconnect from the database.
#[derive(Clone)]
pub struct Database {
    pub(crate) pool: Option<MySqlPool>,
    pub(crate) db_url: String,
}

impl Database {
    pub async fn new(db_url: &str) -> Self {
        Database {
            pool: None,
            db_url: db_url.into(),
        }
    }

    pub async fn connect(&mut self) -> Result<(), sqlx::Error> {
        let pool = MySqlPoolOptions::new().connect(&self.db_url).await?;
        self.pool = Some(pool);
        Ok(())
    }

    pub async fn disconnect(&mut self) {
        if let Some(pool) = &self.pool {
            pool.close().await;
            self.pool = None;
        }
    }

    /// `query` must contain `?` placeholders and `binds` the values to bind, in
    /// order. Values are sent to the server separately from the SQL text, which
    /// makes SQL injection through values impossible.
    pub async fn execute_query(&self, query: &str, binds: &[String]) -> Result<(), sqlx::Error> {
        let pool = self.pool.as_ref().ok_or(sqlx::Error::PoolClosed)?;

        let mut sql = sqlx::query(query);
        for value in binds {
            sql = sql.bind(value.as_str());
        }
        sql.execute(pool).await?;
        Ok(())
    }

    /// `T` must implement [`FromRow`], which sqlx derives for the model structs.
    /// The mapping is done by column name: each field of `T` is filled from the
    /// column bearing the same name.
    pub async fn fetch_all_as<T>(
        &self,
        query: &str,
        binds: &[String],
    ) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
    {
        let pool = self.pool.as_ref().ok_or(sqlx::Error::PoolClosed)?;

        let mut sql = sqlx::query_as::<_, T>(query);
        for value in binds {
            sql = sql.bind(value.as_str());
        }
        sql.fetch_all(pool).await
    }

    /// Same as [`fetch_all_as`](Self::fetch_all_as) but for queries expected to
    /// return zero or one row (e.g. fetching a single record by id, or the most
    /// recent one with `LIMIT 1`).
    pub async fn fetch_optional_as<T>(
        &self,
        query: &str,
        binds: &[String],
    ) -> Result<Option<T>, sqlx::Error>
    where
        T: for<'r> FromRow<'r, MySqlRow> + Send + Unpin,
    {
        let pool = self.pool.as_ref().ok_or(sqlx::Error::PoolClosed)?;

        let mut sql = sqlx::query_as::<_, T>(query);
        for value in binds {
            sql = sql.bind(value.as_str());
        }
        sql.fetch_optional(pool).await
    }

    /// Runs a raw, multi-statement SQL script (e.g. the content of a `.sql` migration file).
    pub async fn execute_script(&self, script: &str) -> Result<(), sqlx::Error> {
        let pool = self.pool.as_ref().ok_or(sqlx::Error::PoolClosed)?;
        sqlx::raw_sql(script).execute(pool).await?;
        Ok(())
    }
}
