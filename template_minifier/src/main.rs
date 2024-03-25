use std::process;

use template_minifier::{run, Config};

fn main() {
    println!("");
    let args = std::env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|error| {
        eprintln!("{:#?}", error);
        process::exit(1);
    });
    if let Err(error) = run(&config) {
        eprintln!("{:#?}", error);
        process::exit(1);
    }
    println!("");

    process::exit(0);
}
