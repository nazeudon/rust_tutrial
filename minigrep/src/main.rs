extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let config = parse_config(&args);
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
