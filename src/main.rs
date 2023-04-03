use std::{collections::HashMap, io::BufRead};
pub(crate) use std::{
    fs::File,
    io::{self, BufReader},
};

#[derive(Clone)]
struct Config {
    file_structure: HashMap<Column, usize>,
    separator: char,
}

// name : date : category : in : out : net
#[derive(Clone, PartialEq, Eq, Hash)]
enum Column {
    Name,
    Date,
    In,
    Out,
    Net,
}

impl Config {
    fn new(bank_type: BankType) -> Self {
        let mut file_structure: HashMap<Column, usize> = HashMap::new();

        match bank_type {
            BankType::LLOYDS => {
                file_structure.insert(Column::Name, 5);
                file_structure.insert(Column::Date, 1);
                file_structure.insert(Column::In, 7);
                file_structure.insert(Column::Out, 6);
                file_structure.insert(Column::Net, 8);
            }
        }

        Self {
            file_structure,
            separator: ',',
        }
    }
}

#[derive(Clone)]
enum BankType {
    LLOYDS,
}

fn main() {
    let config = Config::new(BankType::LLOYDS);
    start(config);
}

fn start(config: Config) {
    loop {
        let mut user_input = String::new();

        println!("Option 1: Upload CSV");
        println!("Option 2: View Current Categories");
        println!("Option 3: View Monthly Spending");
        println!("Choose an option:");

        match io::stdin().read_line(&mut user_input) {
            Ok(_) => (),
            Err(_) => continue,
        };

        match user_input.trim() {
            "1" => upload(&config),
            "2" => view_categories(),
            "3" => view_spending(),
            _ => {
                println!("Unknown option {}", user_input);
                continue;
            }
        }
    }
}

fn upload(config: &Config) {
    loop {
        let mut filepath = String::new();
        println!("Enter the filepath:");

        match io::stdin().read_line(&mut filepath) {
            Ok(_) => (),
            Err(err) => {
                println!("unknown input, {}", err);
                continue;
            }
        }

        let file_buffer = match load_file_into_buffer(filepath.trim()) {
            Ok(f) => f,
            Err(e) => {
                println!("unable to load file buffer {}", e);
                return;
            }
        };

        process_csv(file_buffer, &config);

        return;
    }
}

// need to update config with the specific input
// lloyds | HSBC | Monzo etc etc etc

// what does the data structure for that look like?
// some sort of hash map?
// each of the keys denotes the type
// name : category : date : in : out : net
// each of the values represents the column in which they live
fn process_csv(file_buffer: BufReader<File>, config: &Config) {
    // for each line ( not including the header! )
    // need to loop through -> grab the data -> move that data into array
    for line in file_buffer.lines() {
        let cur_line = match line {
            Ok(l) => l,
            Err(e) => {
                println!("unable to process line, {}", e);
                continue;
            }
        };

        let split_lines: Vec<&str> = cur_line.split(config.separator).collect();
        let name = match config.file_structure.get(&Column::Name) {
            Some(n) => n,
            None => &0,
        };

        let res = split_lines[*name];
        if res.trim() == "" {
            continue;
        }

        println!("{}", res);
    }
}

fn load_file_into_buffer(filepath: &str) -> Result<BufReader<File>, io::Error> {
    let open_file = match File::open(filepath) {
        Ok(file) => file,
        Err(e) => {
            println!("unable to open file, {}", e);
            return Err(e);
        }
    };

    let file_buffer = BufReader::new(open_file);
    return Ok(file_buffer);
}

fn view_categories() {
    println!("View categories");
}

fn view_spending() {
    println!("View monthly spending");
}
