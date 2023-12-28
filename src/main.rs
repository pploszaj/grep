use std::env;
use std::process;
use minigrep::Config;

fn main() {
    //read command line arguments and collect the values into a vector
    let args: Vec<String> = env::args().collect();

    //passing a reference to args
    //handling an error if our Config build function fails and display readable error message to end user
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}





