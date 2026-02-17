/*
3 - Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering”
or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/
use std::collections::HashMap;
use std::io::stdin;
use std::thread::sleep;
use std::time;

pub fn add_name_and_departement(
    // pub keyword only to be able to test it in the main function.
    name: String,
    departement: String,
    map: &mut HashMap<String, String>,
    departement_list: &Vec<String>,
) -> Option<()> {
    if departement_list.contains(&departement) {
        map.insert(name, departement);
    } else {
        return None;
    }

    return Some(());
}

fn sort_and_print_vec(mut vector: Vec<String>) {
    vector.sort();
    for val in vector {
        print!("{val} ");
    }
    println!("");
}

pub fn print_name_per_departement(departement_asked: String, map: &HashMap<String, String>) {
    let mut names: Vec<String> = Vec::new();
    let p: String;
    for (name, departement) in map {
        if *departement == departement_asked {
            names.push(name.to_string());
        }
    }
    sort_and_print_vec(names);
    let sleep_time = time::Duration::from_millis(300);

    sleep(sleep_time);
}

fn print_all_people(map: &HashMap<String, String>) {
    let mut names: Vec<String> = Vec::new();
    for key in map.keys() {
        names.push(key.to_string());
    }
    sort_and_print_vec(names);
    let sleep_time = time::Duration::from_millis(300);

    sleep(sleep_time);
}

pub fn ask_and_add(map: &mut HashMap<String, String>, departement_list: &Vec<String>) {
    let mut answer_departement = String::new();
    let mut answer_name = String::new();

    println!("What is the name of the person you want to assign or replace a department : ");
    stdin()
        .read_line(&mut answer_name)
        .expect("Error reading the line of the name");
    let clean_name = answer_name.trim().to_string();

    println!("What is the name of the departement you want to assign : ");
    stdin()
        .read_line(&mut answer_departement)
        .expect("Error reading the line of the departement");
    let clean_departement = answer_departement.trim().to_string();

    match add_name_and_departement(clean_name, clean_departement, map, departement_list) {
        Some(_) => println!("Success!"),
        None => println!("Error: Department does not exist."),
    }
    let sleep_time = time::Duration::from_millis(300);

    sleep(sleep_time);
}

pub fn ask_and_print(map: &HashMap<String, String>) {
    let mut answer = String::new();
    println!("What is the name of the departement you want to look people that are into : ");
    stdin()
        .read_line(&mut answer)
        .expect("Error reading the line of the departement");

    let clean_answer = answer.trim().to_string();
    println!("{answer}");
    print_name_per_departement(clean_answer, &map);
}

pub fn answer3(map: &mut HashMap<String, String>, departement_list: &Vec<String>) {
    println!("Welcome to the management simulator ! ");
    loop {
        println!(
            "\n\n0 - Quit the program\n1 - Add a new person or assign a new departement\n2 - Display all the peoples at x departement\n3 - Display every persons"
        );
        let mut answer = String::new();

        stdin()
            .read_line(&mut answer)
            .expect("Error while reading the line");

        let answer: i32 = answer
            .trim()
            .parse()
            .expect("error while converting to int");

        match answer {
            0 => break,
            1 => ask_and_add(map, departement_list),
            2 => ask_and_print(map),
            3 => print_all_people(map),
            _ => println!("Your answer was not the one attended"),
        }
    }
}
