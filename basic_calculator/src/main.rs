use std::io;
use std::io::Write;

pub mod math;
use math::operations::{ add, subtract, multiply, divide };

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operations {
    fn apply_operation(&self, lhs: f64, rhs: f64) -> Option<f64> {
        match self {
           Self::Add => add(lhs, rhs),
           Self::Subtract => subtract(lhs, rhs),
           Self::Multiply => multiply(lhs, rhs),
           Self::Divide => divide(lhs, rhs),
        }
    }
}

fn welcome_prompt() {
    println!("Basic Calculator With Operations: '+, -, *, /'");
}

fn prompt(msg: &str) {
    print!("{msg}");
    io::stdout().flush().unwrap();
}

fn get_parsed_number() -> f64 {
    loop {
        let mut number = String::new();
        if io::stdin().read_line(&mut number).is_ok() {
            if let Ok(num) = number.trim().parse::<f64>() {
                return num;
            }
        }
        println!("Invalid Number, Try Again");
    }
}

fn main() {
    welcome_prompt();

    prompt("Enter the first number: ");
    let first_number = get_parsed_number();


    prompt("Enter the second number: ");
    let second_number = get_parsed_number();

    prompt("Enter the operation: ");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("error reading line");
    let operation = operation.trim();

    let op = match operation {
        "+" => Operations::Add,
        "-" => Operations::Subtract,
        "*" => Operations::Multiply,
        "/" => Operations::Divide,
        _ => {
            println!("unknown operation");
            return;
        }
    };

    if let Some(res) = op.apply_operation(first_number, second_number) {
        println!("Result: {res}");
    }
}
