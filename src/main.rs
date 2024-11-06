use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(0);
    });

    if let Err(err) = run(config) {
        eprintln!("error reading contents: {err}");
        process::exit(0);

    }

}