use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // prints to standard error instead
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // if this errors, assign it to Err(e) and run the following lambda
    if let Err(e) = run(config) {
        eprintln!("Application error {e}");
        process::exit(1);
    }
}