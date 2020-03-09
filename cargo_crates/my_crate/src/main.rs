use std::process;

fn main() {
    if let Err(e) = my_crate::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}