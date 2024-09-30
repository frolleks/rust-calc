use std::io;

fn main() {
    println!("Simple Calculator");

    println!("Enter the first number");
    let num1 = read_input().trim().parse::<f64>().unwrap();

    println!("Enter the operator (+, -, *, /)");
    let binding = read_input();
    let operator = binding.trim();

    println!("Enter the second number");
    let num2 = read_input().trim().parse::<f64>().unwrap();

    let result = calculate(num1, num2, operator);

    match result {
        Some(value) => println!("The answer is {}", value),
        None => println!("No answer/invalid"),
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
}

fn calculate(num1: f64, num2: f64, operator: &str) -> Option<f64> {
    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 != 0.0 {
                Some(num1 / num2)
            } else {
                println!("Error: Division by zero!");
                None
            }
        }
        _ => None,
    }
}
