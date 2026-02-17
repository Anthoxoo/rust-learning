/*  those 3 testes are provided by the rust programming book : https://doc.rust-lang.org/book/ch08-03-hash-maps.html

1 - Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map
 will be helpful here) of the list.

2 - Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel
 have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

3 - Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering”
or “Add Amir to Sales.” Then, let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
*/
#![allow(unused)]
use std::array;
use std::collections::HashMap;

mod test1;
use test1::answer;

mod test2;
use test2::answer2;

mod test3;
use test3::answer3;

fn main() {
    // #![allow(unused)]
    // let v = vec![1, 5, 3, 2, 9, 0, 8, 8]; // v sorted : [0, 1, 2, 3, 5, 8, 8, 9]
    // let (median, mode) = answer(v);
    // println!("median : {median}, mode : {mode}");
    // println!("{}", answer2(String::from("éonjour")));
    // let mut map: HashMap<String, String> = HashMap::new();
    // let departement_list: Vec<String> = vec![String::from("BatA"), String::from("BatB")];
    // test3::add_name_and_departement(
    //     String::from("Bonjour"),
    //     String::from("BatA"),
    //     &mut map,
    //     &departement_list,
    // );
    // test3::add_name_and_departement(
    //     String::from("Antoine"),
    //     String::from("BatB"),
    //     &mut map,
    //     &departement_list,
    // );
    // test3::add_name_and_departement(
    //     String::from("Au-revoir"),
    //     String::from("BatA"),
    //     &mut map,
    //     &departement_list,
    // );
    // test3::ask_and_print(&map);
    // answer3(&mut map, &departement_list)
}
