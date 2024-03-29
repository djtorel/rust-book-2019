use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = match scores.get(&team_name) {
        Some(score) => *score,
        _ => 0,
    };

    println!("Blue's score: {}", score);

    let score = match scores.get(&"Yellow".to_string()) {
        Some(score) => *score,
        _ => 0,
    };

    println!("Yellow's score: {}", score);

    let score = match scores.get(&team_name) {
        Some(score) => *score,
        _ => 0,
    };

    println!("Blue's score: {}", score);

}
