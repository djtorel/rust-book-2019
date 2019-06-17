use std::io;

fn main() {
    loop {
        println!("Enter a number (q to quit):");
        let input: u64 = match &*user_input() {
            "q" => break,
            num => match num.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("You must enter a number.");
                    continue;
                }
            },
        };

        println!("Fibonacci {}", fib(input));
    }
}

fn fib(num: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;
    let mut n = num;
    while n > 1 {
        let t = a;
        a = b;
        b += t;
        n -= 1;
    }

    b
}

fn user_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    String::from(input.trim())
}
