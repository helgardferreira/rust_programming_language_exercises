use std::process;
use my_art::{PrimaryColor, mix, SecondaryColor};

fn main() {
    match mix(
        PrimaryColor::Blue,
        PrimaryColor::Yellow,
    ).unwrap_or_else(|err| {
        eprintln!("Unexpected error: {:?}", err);
        process::exit(1);
    }) {
        SecondaryColor::Orange => println!("Orange"),
        SecondaryColor::Green => println!("Green"),
        SecondaryColor::Purple => println!("Purple")
    };
}