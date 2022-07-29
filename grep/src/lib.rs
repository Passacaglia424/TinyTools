use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let outcomes = if config.sensitive {
        search(config.query.as_str(), contents.as_str())
    } else {
        search_on_sensitive(config.query.as_str(), contents.as_str())
    };
    for line in outcomes {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query;
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_on_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub struct Config {
    query: String,
    filename: String,
    sensitive: bool,
}

impl Config {
    pub fn new(query: String, filename: String, sensitive: bool) -> Self {
        Self {
            query,
            filename,
            sensitive,
        }
    }

    pub fn parse_config(args: &[String]) -> Option<Config> {
        //参数数量捕获
        if args.len() < 3 {
            println!("Too few arguments");
            return None;
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case = env::var("CASE_SENSITIVE").is_err(); //PATH
        Some(Config::new(query, filename, case))
    }
}
