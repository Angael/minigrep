// pub struct Config {
//     pub query: String,
//     pub file_path: String,
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

// pub fn run(config: Config) {
//     println!("Query: {:?}", config.query);
//     println!("File name: {:?}", config.file_path);

//     let contents =
//         std::fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

//     println!("Content: {:?}", contents);
// }
