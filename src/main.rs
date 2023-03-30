use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() {
    start();
}

fn start() {
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
            "1" => upload(),
            "2" => view_categories(),
            "3" => view_spending(),
            _ => {
                println!("Unknown option {}", user_input);
                continue;
            }
        }
    }
}

fn upload() {
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
