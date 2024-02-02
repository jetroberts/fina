use chrono::NaiveDateTime;

use crate::{
    database::textfile::TextFile,
    service::transaction::{CreateTransaction, TransactionService},
};

pub struct Cli {
    transaction_service: TransactionService<TextFile>,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            transaction_service: TransactionService::new(TextFile::new()),
        }
    }

    pub fn start(self) {
        let _new_transaction = CreateTransaction {
            account_type: format!("Amex"),
            payment_date: NaiveDateTime::parse_from_str("2021-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                .unwrap(),
            amount: 12.34,
            description: format!("TEST"),
        };
    }
}
