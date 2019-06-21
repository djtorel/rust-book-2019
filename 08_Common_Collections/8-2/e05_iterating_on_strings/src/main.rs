fn main() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

    // There are currently no tools for iterating grapheme clusters in the
    // standard library. Look for crates to do this if necessary.
}
