// file: src/database/connection.rs
// description: Database connection management

use crate::types::errors::DatabaseError;
use crate::types::AppResult;
use libsql::{Builder, Connection};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

pub struct DatabaseConnection {
    conn: Arc<RwLock<Connection>>,
}

impl DatabaseConnection {
    pub async fn new(db_path: &str) -> AppResult<Self> {
        info!("Connecting to database: {}", db_path);

        let db = Builder::new_local(db_path)
            .build()
            .await
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        let conn = db
            .connect()
            .map_err(|e| DatabaseError::Connection(e.to_string()))?;

        let connection = Self {
            conn: Arc::new(RwLock::new(conn)),
        };

        // Initialize schema
        super::schema::initialize(&connection).await?;

        Ok(connection)
    }

    pub async fn execute(&self, sql: &str, params: libsql::params::Params) -> AppResult<u64> {
        let conn = self.conn.write().await;
        conn.execute(sql, params)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()).into())
    }

    pub async fn query(
        &self,
        sql: &str,
        params: libsql::params::Params,
    ) -> AppResult<libsql::Rows> {
        let conn = self.conn.read().await;
        conn.query(sql, params)
            .await
            .map_err(|e| DatabaseError::Query(e.to_string()).into())
    }
}
