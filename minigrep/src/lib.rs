pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn parse_config(args: &[String]) -> Result<Config, &str> {
        if args.len() > 3 {
            return Err("Too much argument : 'minigrep [query] [file_path]'");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        return Ok(Config { query, file_path });
    }
}
