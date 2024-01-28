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
        let new_transaction = CreateTransaction {
            account_type: format!("Amex"),
            date: format!("01/01/2022"),
            amount: 12.34,
            description: format!("TEST"),
        };

        let _ = self.transaction_service.save_transaction(new_transaction);
    }
}
