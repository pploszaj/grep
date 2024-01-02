use std::env;
use std::process;
use minigrep::Config;

fn main() {
    //passing a reference to args
    //handling an error if our Config build function fails and display readable error message to end user
    //env::args returns an interator, so we're passing ownsership of the iteratorn to config::build directly
    let config = Config::build(env::args()).unwrap_or_else(|err| {
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





