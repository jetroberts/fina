use std::{fs::File, io::Read};

use crate::{
    database::textfile::TextFile,
    service::{
        parse::{Config, ParsedTransaction, Service},
        transaction::TransactionService,
    },
};

pub struct Cli {
    parse_service: Service,
    transaction_service: TransactionService<TextFile>,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            parse_service: Service::new(),
            transaction_service: TransactionService::new(TextFile::new()),
        }
    }

    pub fn start(self) {
        let new_transaction = ParsedTransaction::test();

        let _ = self.transaction_service.save_transaction(new_transaction);
    }
}
