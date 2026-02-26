use std::env;
use std::fs;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect(); // args() is an iterator and collect() turn what it went through into a vector
    let config: Config = Config::parse_config(&args).expect("Problem while parsing the arguments");

    run_program(config.file_path);
}

fn run_program(file_destination: String) {
    let file_content =
        fs::read_to_string(file_destination).expect("Couldn't read {file_destination}");
    println!("{file_content}");
}
