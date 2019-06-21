use std::io;

fn main() {
    let mut string = String::new();

    println!("Enter the sentence to turn into Pig-Latin");
    io::stdin()
        .read_line(&mut string)
        .expect("Failed to read input");

    let string = String::from(string.trim());
    println!("new_string: {}", pig_latin(string));
}

fn pig_latin(string: String) -> String {
    // Converting string to Vec<&str> splitting words by white space
    let string_vec: Vec<&str> = string.split_whitespace().collect();

    // Creating a new string, iterating over words in string_vec,
    // mutating the word, then pushing it to the new string
    let mut new_string = String::new();
    for &word in &string_vec {
        let new_word = mutate_word(word);

        new_string.push_str(&new_word);
    }

    // returning pig-latinified word
    String::from(new_string.trim())
}

// mutate_word function takes in a word, tuend into a Vec<char> to manipulate,
// individual chars. Then checks if the first char is a vowel, if it is a vowl
// it pushes 'h' to the end of the chars vector, if not it removes the first char,
// pushes it to the end of the vec.
//
// One it has the first char figured out, it combines chars into a String
//Then pushes "ay " to the end of the word. Finally returns the mutated word
fn mutate_word(word: &str) -> String {
    // Split word into Vec<char> of individual chars then push '-' at end
    let mut chars: Vec<char> = word.chars().collect();
    chars.push('-');
    // Check if first char is vowel, then perform char manipulation based on that
    match is_vowel(word) {
        true => chars.push('h'),
        false => {
            let char1 = chars.remove(0);
            chars.push(char1);
        }
    }

    // Combine chars into new_word as String, then push "ay " to end
    let mut new_word: String = chars.into_iter().collect();
    new_word.push_str("ay ");

    new_word
}

// Function to check if word starts with a vowel
fn is_vowel(word: &str) -> bool {
    match word.to_lowercase().chars().next() {
        Some('a') => true,
        Some('e') => true,
        Some('i') => true,
        Some('o') => true,
        Some('u') => true,
        _ => false,
    }
}
