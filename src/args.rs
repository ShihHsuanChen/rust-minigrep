use std;


pub struct Arguments {
    pub query: String,
    pub file_path: String,
}

pub enum ValidateResult<T> {
    Help,
    Ok(T),
    Fail(String),
}


impl Arguments {
    pub fn print_help() {
        let prog = String::from("minigrep");
        let desc = String::from("Match a pattern in a file and print.");
        println!("{prog}");
        println!();
        println!("  {desc}");
        println!();
        println!("Usage: {prog} [OPTIONS] <query> <file_path>");
        println!();
        println!("Options:");
        println!("  -h, --help         print this help message.");
        println!();
        println!("Positional:");
        println!("  query     (string) regular expression pattern.");
        println!("  file_path (string) path to the file to search.");
    }

    pub fn parse_args() -> ValidateResult<Self> {
        // read arguments from command line
        let args: Vec<String> = std::env::args().collect();

        if args.contains(&String::from("--help")) || args.contains(&String::from("-h")) {
            return ValidateResult::Help;
        }
        if args.len() != 3 {
            return ValidateResult::Fail(format!("Requires 2 arguments but {} given", args.len() - 1));
        }

        // parse arguments and validate
        let query = match Self::validate_query(&args[1]) {
            ValidateResult::Ok(v) => v,
            ValidateResult::Fail(errmsg) => {
                return ValidateResult::Fail(errmsg);
            },
            _ => String::from(""),
        };

        let file_path = match Self::validate_file_path(&args[2]) {
            ValidateResult::Ok(v) => v,
            ValidateResult::Fail(errmsg) => {
                return ValidateResult::Fail(errmsg);
            },
            _ => String::from(""),
        };
        ValidateResult::Ok(Arguments { query, file_path })
    }
    
    fn validate_query(query: &String) -> ValidateResult<String> {
        ValidateResult::Ok(query.to_string())
    }

    fn validate_file_path(file_path: &String) -> ValidateResult<String> {
        let path = std::path::Path::new(file_path);
        if !path.exists() {
            ValidateResult::Fail(format!("{file_path} does not exist"))
        } else if !path.is_file() {
            ValidateResult::Fail(format!("{file_path} is not a file"))
        } else {
            ValidateResult::Ok(file_path.to_string())
        }
    }
}

