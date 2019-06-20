enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    println!("Hello, world!");
}

fn value_in_cents(coin: Coin) -> u8 {
    match Coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
