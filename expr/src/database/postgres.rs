use std::sync::Arc;

use sqlx::PgPool;
use uuid::Uuid;

use crate::service::transaction::{
    CreateTransaction, Transaction, TransactionRead, TransactionWrite,
};

use super::base::{DatabaseError, DatabaseInit, DatabaseRead, DatabaseWrite};

pub struct Postgres {
    connection_string: Arc<str>,
    pool: Option<PgPool>,
}

impl Postgres {
    pub fn new(connection_string: &str) -> Self {
        Self {
            pool: None,
            connection_string: connection_string.into(),
        }
    }
}

impl DatabaseInit for Postgres {
    async fn connect(&mut self) -> Result<(), DatabaseError> {
        println!("Connecting to postgres");
        let pool = match PgPool::connect(self.connection_string.as_ref()).await {
            Ok(pool) => pool,
            Err(e) => {
                eprintln!("Error connecting to postgres: {}", e);
                return Err(DatabaseError::ConnectionError(e.to_string()));
            }
        };

        println!("Connected to postgres");

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
        _id: &str,
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
        _record: T,
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

    async fn delete(&mut self, _id: &str) -> Result<bool, DatabaseError> {
        todo!()
    }
}

impl TransactionWrite for Postgres {
    async fn create_transaction(
        &mut self,
        create_transaction: CreateTransaction,
    ) -> Result<Uuid, DatabaseError> {
        if self.pool.is_none() {
            self.connect().await?;
        }

        if let Some(pool) = &self.pool {
            let res = sqlx::query!(
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

            return Ok(res.id);
        }

        return Err(DatabaseError::ConnectionError("No connection".to_string()));
    }

    async fn delete_transactions(&mut self) -> Result<(), DatabaseError> {
        todo!();
    }
}

impl TransactionRead for Postgres {
    async fn get_transaction(&self, id: &str) -> Result<Option<Transaction>, DatabaseError> {
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

            Ok(Some(record))
        }

        Err(DatabaseError::GetError("No connection".to_string()))
    }
}
