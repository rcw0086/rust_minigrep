// use minigrep::Config; // vscode error, workaround with lib import
mod lib;
use lib::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = lib::run(config) {
        eprintln!("Application err: {}", e);
        process::exit(1);
    }
}
