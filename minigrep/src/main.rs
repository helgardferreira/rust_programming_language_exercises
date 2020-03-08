use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // calling Config::new and handling errors
    // using the returned iterator directly
    let config = Config::new(env::args())
        .unwrap_or_else(|err| {
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        });

    // handling errors returned from run in main
    /*
    We use if let here instead of unwrap_or_else since we don't care about
    run's return value.
    */
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
