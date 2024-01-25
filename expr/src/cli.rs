use std::{fs::File, io::Read};

use crate::service::parse::{Config, Service};

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
        let mut file = File::open("test.csv").expect("Could not find / open test file");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Could not read test file");

        let _res = self.parse_service.parse_data(
            Config {
                name: "Test".to_string(),
                date_position: 2,
                amount_position: 3,
                description_position: 4,
            },
            buf,
        );
    }
}
