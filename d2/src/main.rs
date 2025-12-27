use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter first number:");
    io::stdin().read_line(&mut input).unwrap();
    let a: f64 = input.trim().parse().unwrap();

    input.clear();

    println!("Enter operator (+, -, *, /):");
    io::stdin().read_line(&mut input).unwrap();
    let op = input.trim().chars().next().unwrap();

    input.clear();

    println!("Enter second number:");
    io::stdin().read_line(&mut input).unwrap();
    let b: f64 = input.trim().parse().unwrap();

    let result = match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => return,
    };

    println!("Result: {}", result);
}
