use rand::Rng;
use std::collections::HashMap;

fn main() {
    // Generate arbitrary amount of random numbers to compute
    let mut numbers = Vec::new();
    const AMOUNT: u32 = 30000;
    const MIN: u32 = 0;
    const MAX: u32 = 100000;

    println!("Generating {} integers between {} and {}", AMOUNT, MIN, MAX);

    for _ in 0..AMOUNT {
        let num: u32 = rand::thread_rng().gen_range(MIN, MAX);
        numbers.push(num);
    }

    // Find average by calculating sum and divide by amount
    let mut sum: u32 = 0;
    for &number in &numbers {
        sum += number;
    }
    let avg = sum / AMOUNT;
    println!("Average: {} / {} = {}", sum, AMOUNT, avg);

    // Find the median by sorting vector then find middle position
    numbers.sort();
    let median = match numbers.get((AMOUNT / 2) as usize) {
        Some(num) => *num,
        _ => 0,
    };
    println!("Median is: {}", median);

    // Find mode by counting the number of duplicates in a HashMap
    let mut number_hash = HashMap::new();
    for &number in &numbers {
        let count = number_hash.entry(number).or_insert(0);
        *count += 1;
    }

    let mut largest_val = 0;
    let mut mode = 0;
    for (key, val) in number_hash.iter() {
        if *val > largest_val {
            largest_val = *val;
            mode = *key;
        }
    }

    println!("Mode: {}", mode);
}
