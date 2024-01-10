use std::fs::File;

use crate::service::service::{AccountType, Config, Service};

pub struct Cli {
    parse_service: Service,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            parse_service: Service::new(),
        }
    }

    pub fn start(self) {
        let file = File::open("test.csv").expect("Could not find / open test file");
        let _res = self.parse_service.parse(
            AccountType::Test(Config {
                name: "Test".to_string(),
                date: 2,
                amount: 3,
            }),
            file,
        );
    }
}
