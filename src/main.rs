use std::env;
use std::fs;

fn main() {
    //read command line arguments and collect the values into a vector
    let args: Vec<String> = env::args().collect();

    //passing a reference to args
    let config = Config::new(&args);
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let content = fs::read_to_string(config.file_path).expect("Error reading file");

    println!("With text:\n{content}");
}

struct Config {
    query: String,
    file_path: String
}
//By cloning, you create a new String that is a copy of the string data. This means Config now owns its data independently of the args vector
impl Config {
    fn build(args: &Vec<String>) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
}
