use std::env;

use minigrep::{run, Config};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let config = parse_config(&args);

    run(config);
}

fn parse_config(args: &[String]) -> Config {
    Config {
        query: args[1].clone(),
        file_path: args[2].clone(),
    }
}
