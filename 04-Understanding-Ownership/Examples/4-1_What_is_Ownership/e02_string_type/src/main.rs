fn main() {
    // The String::from() function allocates memory on the heap for a growable
    // and shrinkable piece of memory to contain the data in the string
    let mut s = String::from("hello"); // s is valid from this point forward

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print 'hello, world'
} // This scope is over, and s is no longer
  // valid. Once s goes out of scope, the memory for the data associated with it
  // gets de-allocated.
