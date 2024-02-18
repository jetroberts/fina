use std::{
    fmt::{self, Display},
    sync::Arc,
};

use chrono::{NaiveDate, NaiveDateTime};
use tokio::sync::RwLock;

use crate::{database::postgres::Postgres, models::transaction::CreateTransaction};

use super::transaction::TransactionService;

pub struct Service {
    transaction_service: Arc<RwLock<TransactionService<Postgres>>>,
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
    DateConversionError(String),
    SaveError(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::RecordError(e) => write!(f, "RecordError: {}", e),
            ParseError::AmountConversionError(e) => write!(f, "AmountConversionError: {}", e),
            ParseError::SaveError(e) => write!(f, "SaveError: {}", e),
            ParseError::DateConversionError(e) => write!(f, "DateConversionError: {}", e),
        }
    }
}

impl Service {
    pub fn new(transaction_service: Arc<RwLock<TransactionService<Postgres>>>) -> Self {
        Self {
            transaction_service,
        }
    }

    pub async fn parse_data(
        &self,
        extraction_config: Config,
        data: String,
    ) -> Result<(), ParseError> {
        let mut csv_reader = csv::Reader::from_reader(data.as_bytes());
        let transaction_service = self.transaction_service.write().await;

        for (index, record) in csv_reader.records().enumerate() {
            if index == 0 {
                continue;
            }

            let r = record.map_err(|e| ParseError::RecordError(e.to_string()))?;

            let date = string_or_empty(r.get(extraction_config.date_position));
            let amount = string_or_empty(r.get(extraction_config.amount_position));
            let description = string_or_empty(r.get(extraction_config.description_position));

            let amount = amount
                .parse::<f64>()
                .map_err(|e| ParseError::AmountConversionError(e.to_string()))?;

            let pd = NaiveDate::parse_from_str(date, "%d/%m/%Y")
                .map_err(|e| ParseError::DateConversionError(e.to_string()))?;

            let empty_time = match chrono::NaiveTime::from_hms_opt(0, 0, 0) {
                Some(time) => time,
                None => return Err(ParseError::DateConversionError("Invalid time".to_string())),
            };

            let new_transaction = CreateTransaction {
                account_type: extraction_config.name.clone(),
                amount,
                payment_date: NaiveDateTime::new(pd, empty_time),
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
