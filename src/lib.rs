pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub fn run(config: Config) {
    println!("Query: {:?}", config.query);
    println!("File name: {:?}", config.file_path);

    let contents =
        std::fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

    println!("Content: {:?}", contents);
}
