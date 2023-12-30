pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// cargo run -- --insensitive-case --file poem.txt --query the
// cargo run -- -i -f poem.txt -q the
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut query: Option<String> = None;
        let mut file_path: Option<String> = None;
        let mut ignore_case = false;

        let mut i = 0;
        for arg in args {
            match arg.as_str() {
                "-i" | "--insensitive-case" => {
                    ignore_case = true;
                }
                "-q" | "--query" if (i + 1) < args.len() => {
                    query = Some(args[i + 1].clone());
                }
                "-f" | "--file" if (i + 1) < args.len() => {
                    file_path = Some(args[i + 1].clone());
                }
                _ => {}
            }
            i += 1;
        }

        if query == None {
            return Err("Query is required. Please provide a query with -q or --query");
        }

        if file_path == None {
            return Err("File path is required. Please provide a file path with -f or --file");
        }

        return Ok(Config {
            query: query.unwrap(),
            file_path: file_path.unwrap(),
            ignore_case,
        });
    }
}
