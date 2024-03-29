struct ImportantExcept<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not finda a '.'");

    let i = ImportantExcept {
        part: first_sentence,
    };

    println!("Excerpt: {}", i.part);
}
