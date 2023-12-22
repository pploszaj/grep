use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    //read command line arguments and collect the values into a vector
    let args: Vec<String> = env::args().collect();

    //passing a reference to args
    //handling an error if our Config build function fails and display readable error message to end user
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    run(config);
}

fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.file_path)?;
    println!("With text:\n{content}");
    Ok(())
}

struct Config {
    query: String,
    file_path: String
}
//By cloning, you create a new String that is a copy of the string data. This means Config now owns its data independently of the args vector
impl Config {
    //The error is a string slice, not an owned String. This is often used for error messages to avoid the overhead of heap allocation associated with String.
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}
