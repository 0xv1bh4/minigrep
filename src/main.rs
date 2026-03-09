use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::{search, search_case_insensitive};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing the arguements: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    };
}

struct Config {
    query: String,
    file_name: String,
    ignore_case: bool
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguements");
        }
        let query = args[1].clone();
        let file_name = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {query, file_name, ignore_case})
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_name)?;

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
 