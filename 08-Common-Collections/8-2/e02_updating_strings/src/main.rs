fn main() {
    let mut s = String::from("foo");
    s.push_str("bar");

    println!("s: {}", s);

    s = s + " baz";
    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');
    println!("s is {}", s);
}
