fn main() {
    let len = String::from("Hola").len();
    println!("{}", len);

    let len = String::from("Здравствуйте").len();
    println!("{}", len);

    // You cannot index directly into strings because of how they are stored in
    // memory.

    // If you must index directly into Strings, you must do it via slice:

    let s1 = "Hola";
    let s2 = "Здравствуйте";

    println!("&s1[0..2]: {}", &s1[0..2]);
    println!("&s2[0..2]: {}", &s2[0..2]);
}
