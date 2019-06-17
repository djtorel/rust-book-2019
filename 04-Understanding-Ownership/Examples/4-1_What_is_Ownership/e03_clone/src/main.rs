fn main() {
    let s1 = String::from("hello"); // new string created
    let s2 = s1.clone(); // copy of this string is allocated
                         // and created and assigned to s2

    println!("s1 = {}, s2 = {}", s1, s2); // s1 and s2 can be used because
                                          // both own independant pieces of
                                          // memory
}
