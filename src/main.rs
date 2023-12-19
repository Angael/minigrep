use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run(config);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) {
    println!("Query: {:?}", config.query);
    println!("File name: {:?}", config.file_path);

    let contents =
        std::fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("Content: {:?}", contents);
}
