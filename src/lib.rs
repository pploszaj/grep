use std::fs;
use std::error::Error;
use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
//By cloning, you create a new String that is a copy of the string data. This means Config now owns its data independently of the args vector
impl Config {
    //The error is a string slice, not an owned String. This is often used for error messages to avoid the overhead of heap allocation associated with String.
    pub fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config { query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &content)
    } else {
        search(&config.query, &content)
    };

    for line in results{
        println!("{line}");
    }
    Ok(())
}

//Rust has a helpful method to handle line-by-line iteration of strings, conveniently named lines

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}