use anyhow::Result;
use arangors::{Connection, Database};

#[derive(Clone)]
pub struct ArangoClient {
    db: Database<Connection>,
}

impl ArangoClient {
    pub async fn new(url: &str, db_name: &str) -> Result<Self> {
        let conn = Connection::establish_jwt(url, "root", "changeme").await?;
        let db = conn.db(db_name).await?;
        Ok(Self { db })
    }

    pub fn database(&self) -> &Database<Connection> {
        &self.db
    }
}
