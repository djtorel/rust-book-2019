use std::error::Error;
use std::fs::File;

// Doesn't compile because when using `?` the function you use it in must
// return Result<T, E>
// fn main() {
//     let f = File::open("hello.txt")?;
// }

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
