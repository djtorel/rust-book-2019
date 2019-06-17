fn main() {
    let x = 50;

    let x = x + 3;

    let x = x * 2;

    println!(
        "The value of x is: \nDecimal: {num}\nOctal: {num:o}\nHex: {num:x}\nBinary: {num:b}\n\n",
        num = x
    );

    let x = 2.0;

    let y: f32 = 3.0;

    println!("x: {}, y: {}", x, y);
}
