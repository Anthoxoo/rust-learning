use std::env;
use std::fs;
use std::io::Error;
use std::process;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect(); // args() is an iterator and collect() turn what it went through into a vector
    let config: Config = Config::parse_config(&args).expect("Problem while parsing the arguments");

    match run_program(config.query, config.file_target) {
        Err(e) => {
            println!("Error running the command : {e}");
            process::exit(1);
        }

        Ok(_) => {}
    }
}

fn run_program(query: String, target_file: String) -> Result<(), Error> {
    let file_content = fs::read_to_string(&target_file).expect("Couldn't read {file_destination}");
    let result_command: Vec<&str> = minigrep::search(&query, &file_content);
    if result_command.len() == 0 {
        println!("");
    } else {
        for value in result_command {
            println!("{value}")
        }
    }
    Ok(())
}
