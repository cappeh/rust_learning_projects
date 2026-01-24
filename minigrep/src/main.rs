use std::{env, fs, process};
use std::error::Error;

use minigrep::{search, search_case_insensitive};

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didnt get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        // if args.contains(&"--case-sensitive".to_string()) {
        //     ignore_case = false;
        // } else if args.contains(&"--ignore-case".to_string()) {
        //    ignore_case = true;
        // }

        Ok(Config { query, file_path, ignore_case })
    }
}
