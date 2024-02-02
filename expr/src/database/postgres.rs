use sqlx::PgPool;
use uuid::Uuid;

use crate::service::transaction::{CreateTransaction, Transaction};

use super::base::{DatabaseError, DatabaseInit, DatabaseRead, DatabaseWrite};

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
        let pool = PgPool::connect("")
            .await
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        self.pool = Some(pool);

        Ok(())
    }

    async fn disconnect(&mut self) -> Result<(), DatabaseError> {
        if let Some(pool) = &self.pool {
            pool.close().await;
            self.pool = None;
            return Ok(());
        }

        return Ok(());
    }
}

impl DatabaseRead for Postgres {
    async fn find<T: for<'a> serde::Deserialize<'a>>(
        &mut self,
        id: &str,
    ) -> Result<Option<T>, DatabaseError> {
        todo!()
    }

    async fn find_all<T: for<'a> serde::Deserialize<'a> + Send>(
        &mut self,
    ) -> Result<Vec<T>, DatabaseError> {
        todo!()
    }
}

impl DatabaseWrite for Postgres {
    async fn save<T: ToString + serde::Serialize>(
        &mut self,
        record: T,
    ) -> Result<String, DatabaseError> {
        if self.pool.is_some() {
            self.connect().await?;
        }

        // somehow need to have a save query that can be used for all tables

        // here the problem is that we need to know the table name and the columns
        // so we can build the query
        // each service should have a query that allows it to pull data

        return Ok("".to_string());
    }

    async fn delete(&mut self, id: &str) -> Result<bool, DatabaseError> {
        todo!()
    }
}

impl Postgres {
    async fn create_transaction(
        &self,
        create_transaction: CreateTransaction,
    ) -> Result<(), DatabaseError> {
        if let Some(pool) = &self.pool {
            let _res = sqlx::query!(
                r#"
            INSERT INTO payment_transactions (account_type, payment_date, description, amount)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
                create_transaction.account_type,
                create_transaction.payment_date,
                create_transaction.description,
                create_transaction.amount
            )
            .fetch_one(pool)
            .await
            .map_err(|e| DatabaseError::SaveError(e.to_string()))?;

            return Ok(());
        }

        return Err(DatabaseError::ConnectionError("No connection".to_string()));
    }

    async fn get_transaction(&self, id: &str) -> Result<Transaction, DatabaseError> {
        let id = Uuid::parse_str(id).map_err(|e| DatabaseError::GetError(e.to_string()))?;

        if let Some(pool) = &self.pool {
            let record = sqlx::query_as!(
                Transaction,
                r#"
            SELECT * FROM payment_transactions WHERE id = $1
            "#,
                id
            )
            .fetch_one(pool)
            .await
            .map_err(|e| DatabaseError::GetError(e.to_string()))?;

            println!("{:?}", record);
        }

        Err(DatabaseError::GetError("No connection".to_string()))
    }
}
