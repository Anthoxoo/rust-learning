pub struct Config {
    pub query: String,
    pub file_target: String,
}

impl Config {
    pub fn parse_config(args: &[String]) -> Result<Config, &str> {
        if args.len() > 3 || args.len() <= 1 {
            // args must looks like ["target/debug/minigrep", "option1", "option2"]
            return Err("Usage : 'minigrep [query] [file_path]'");
        }
        let query = args[1].clone();
        let file_target = args[2].clone();

        return Ok(Config { query, file_target });
    }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut line_contains_query: Vec<&str> = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            line_contains_query.push(line)
        }
    }

    return line_contains_query;
}
