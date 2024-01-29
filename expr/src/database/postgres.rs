use sqlx::PgPool;

use super::base::{DatabaseError, DatabaseInit};

pub struct Postgres {
    pool: Option<PgPool>,
}

impl Postgres {
    pub fn new() -> Self {
        Self { pool: None }
    }
}

impl DatabaseInit for Postgres {
    async fn connect(&mut self) -> Result<(), DatabaseError> {
        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), DatabaseError> {
        todo!()
    }
}
