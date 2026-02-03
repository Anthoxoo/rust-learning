use rand::{self, Rng};
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    println!("** Welcome to the guessing game ! **");
    let rng = rand::thread_rng().gen_range(0..10);
    println!("random : {}", rng);
    let mut tries: u16 = 0;
    loop {
        println!("Choose a number between 0 and 10");
        let mut answer = String::new();

        stdin()
            .read_line(&mut answer)
            .expect("Error while reading the line");

        let answer: i32 = answer
            .trim()
            .parse()
            .expect("error while converting to int");
        tries += 1;
        println!("Current number of tries : {}", tries);

        // match is a sort of enhanced switch, very useful !
        match answer.cmp(&rng) {
            Ordering::Less => println!("too low !"),
            Ordering::Greater => println!("too high !"),
            Ordering::Equal => break,
        }
    }
    println!("Good job ! You found the number that was : {}", rng);
}
