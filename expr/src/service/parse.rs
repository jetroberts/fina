use std::{
    fmt::{self, Display},
    sync::Arc,
};

use chrono::NaiveDateTime;
use tokio::sync::RwLock;

use crate::database::redis::Redis;

use super::transaction::{CreateTransaction, TransactionService};

pub struct Service {
    transaction_service: Arc<RwLock<TransactionService<Redis>>>,
}

// column position of the given columns
pub struct Config {
    pub name: String,
    pub description_position: usize,
    pub date_position: usize,
    pub amount_position: usize,
}

pub enum ParseError {
    RecordError(String),
    AmountConversionError(String),
    RwLockError(String),
    SaveError(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::RecordError(e) => write!(f, "RecordError: {}", e),
            ParseError::AmountConversionError(e) => write!(f, "AmountConversionError: {}", e),
            ParseError::RwLockError(e) => write!(f, "RwLockError: {}", e),
            ParseError::SaveError(e) => write!(f, "SaveError: {}", e),
        }
    }
}

impl Service {
    pub fn new() -> Self {
        let db = Redis::new();
        Self {
            transaction_service: Arc::new(RwLock::new(TransactionService::new(db))),
        }
    }

    pub async fn parse_data(
        &self,
        extraction_config: Config,
        data: String,
    ) -> Result<(), ParseError> {
        let mut csv_reader = csv::Reader::from_reader(data.as_bytes());
        let transaction_service = self.transaction_service.write().await;

        for record in csv_reader.records() {
            let r = record.map_err(|e| ParseError::RecordError(e.to_string()))?;

            let date = string_or_empty(r.get(extraction_config.date_position));
            let amount = string_or_empty(r.get(extraction_config.amount_position));
            let description = string_or_empty(r.get(extraction_config.description_position));

            let amount = amount
                .parse::<f64>()
                .map_err(|e| ParseError::AmountConversionError(e.to_string()))?;

            let new_transaction = CreateTransaction {
                account_type: extraction_config.name.clone(),
                amount,
                payment_date: NaiveDateTime::parse_from_str(date, "%Y/%m/%d")
                    .map_err(|e| ParseError::SaveError(e.to_string()))?,
                description: description.to_string(),
            };

            transaction_service
                .create_transaction(new_transaction)
                .await
                .map_err(|e| ParseError::SaveError(e.to_string()))?;
        }

        Ok(())
    }
}

fn string_or_empty(s: Option<&str>) -> &str {
    match s {
        Some(value) => value,
        None => "",
    }
}
