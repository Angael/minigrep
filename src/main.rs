use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if cfg!(debug_assertions) {
        println!("Args: {:?}", args);
    }

    let config = minigrep::Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
