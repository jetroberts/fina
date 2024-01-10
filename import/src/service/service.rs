use std::{
    fmt::{self, Display},
    fs::File,
};

pub struct Service;

#[derive(Debug)]
pub struct ParsedTransaction {
    accounttype: String,
    date: String,
    amount: f64,
}

impl Display for ParsedTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "account_type: {}, date: {}, amount: {}",
            self.accounttype, self.date, self.amount
        )
    }
}

// column position of the given columns
pub struct Config {
    pub name: String,
    pub date: usize,
    pub amount: usize,
}

pub enum AccountType {
    Test(Config),
    Lloyds(Config),
}

impl Service {
    pub fn new() -> Self {
        Self {}
    }

    pub fn parse(
        &self,
        account_type: AccountType,
        file: File,
    ) -> Result<Vec<ParsedTransaction>, String> {
        let conf = match account_type {
            AccountType::Test(conf) => conf,
            AccountType::Lloyds(conf) => conf,
        };

        let mut csv_reader = csv::Reader::from_reader(file);
        let mut parsed_transactions: Vec<ParsedTransaction> = Vec::new();
        for result in csv_reader.records() {
            let r = result.unwrap();
            let date = r.get(conf.date).expect("Could not find date");
            let amount = r.get(conf.amount).expect("Could not find amount");

            parsed_transactions.push(ParsedTransaction {
                accounttype: conf.name.clone(),
                date: date.to_string(),
                amount: amount.parse::<f64>().unwrap(),
            })
        }

        Ok(parsed_transactions)
    }
}
