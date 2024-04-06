use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::Config::build(&args).unwrap_or_else(|_| {
        process::exit(1);
    });

    if let Err(_) = minigrep::run(config) {
        process::exit(1);
    }
}
