use std::collections::HashMap;

fn main() {
    // Overwriting values:
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:#?}", scores);
    // -- End overwriting values

    // Only inserting a value if the key has no value:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:#?}", scores);
    // -- End inserting value if the key has no value

    // Updaing a value based on the old value:
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", map);
    // -- End updating value based on the old value
}
