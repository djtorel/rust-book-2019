enum SpeadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpeadsheetCell::Int(3),
        SpeadsheetCell::Text(String::from("blue")),
        SpeadsheetCell::Float(10.12),
    ];
}
