use std::env;
use std::fs;
use std::io::Error;
use std::process;

use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect(); // args() is an iterator and collect() turn what it went through into a vector
    let config: Config = Config::parse_config(&args).expect("Problem while parsing the arguments");

    let program = run_program(config.file_path);
    match program {
        Err(e) => {
            println!("Error running the command : {e}");
            process::exit(1);
        }

        Ok(_) => {}
    }
}

fn run_program(file_destination: String) -> Result<(), Error> {
    let file_content =
        fs::read_to_string(file_destination).expect("Couldn't read {file_destination}");
    println!("{file_content}");
    Ok(())
}
