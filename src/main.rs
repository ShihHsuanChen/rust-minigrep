use std;
use regex::Regex;
use crate::args::{Arguments, ValidateResult};

pub mod args;

fn main() {
    let args = match Arguments::parse_args() {
        ValidateResult::Help => {
            Arguments::print_help();
            std::process::exit(1);
        },
        ValidateResult::Ok(v) => v,
        ValidateResult::Fail(errmsg) => {
            eprintln!("{}", errmsg);
            std::process::exit(1);
        },
    };

    let re = match Regex::new(&args.query) {
        Ok(v) => v,
        Err(_) => {
            println!("Invalid pattern");
            std::process::exit(1);
        },
    };

    // read file content
    let content = std::fs::read_to_string(&args.file_path)
        .expect("Cannot read file {file_path}");

    // let mut results = vec![];
    for (i, line) in content.lines().enumerate() {
        let lineno = i + 1;
        if let Some(caps) = re.captures(&line) {
            // let range = caps.get(0).unwrap().range();
            println!("{lineno}:{line}");
        }
    }
}
