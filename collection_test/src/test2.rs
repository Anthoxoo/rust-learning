use std::array;

/*2 - Convert strings to Pig Latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that
start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
*/
pub fn answer2(word: String) -> String {
    let vowels: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
    let first_letter = word
        .chars()
        .nth(0)
        .expect("Error reading first character of the word.");
    let mut final_word: String = word;
    if vowels.contains(&first_letter) {
        final_word = format!("{final_word}-hay");
    } else if first_letter.is_alphabetic() {
        let first_consonant_value: char = final_word
            .chars()
            .nth(0)
            .expect("Error reading first letter of the word.");
        final_word = format!(
            "{}-{first_consonant_value}ay",
            final_word.chars().skip(1).collect::<String>()
        );
    }
    return final_word;
}
