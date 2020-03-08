use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // accepting command line arguments
    let args: Vec<String> = env::args().collect();

    // calling Config::new and handling errors
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // handling errors returned from run in main
    /*
    We use if let here instead of unwrap_or_else since we don't care about
    run's return value.
    */
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
