use std::fs;
use std::error::Error;
pub struct Config {
    pub query: String,
    pub file_path: String
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
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    println!("With text:\n{content}");
    Ok(())
}