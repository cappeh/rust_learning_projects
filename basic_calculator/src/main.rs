use std::io;
use std::io::Write;

pub mod math;
use math::operations::{ add, subtract, multiply, divide };

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

impl Operation {
    fn apply(&self, lhs: f64, rhs: f64) -> Option<f64> {
        match self {
           Self::Add => Some(add(lhs, rhs)),
           Self::Subtract => Some(subtract(lhs, rhs)),
           Self::Multiply => Some(multiply(lhs, rhs)),
           Self::Divide => divide(lhs, rhs),
        }
    }
}

fn welcome_prompt() {
    println!("Basic Calculator (+, -, *, /)");
    println!("type 'quit' to exit");
}

fn prompt(msg: &str) {
    print!("{msg}");
    io::stdout().flush().unwrap();
}

fn get_parsed_number() -> Option<f64> {
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_ok() {
            if input.trim() == "quit" {
                return None;
            }
            if let Ok(num) = input.trim().parse::<f64>() {
                return Some(num);
            }
        }
        println!("Invalid Number, Try Again");
        prompt("Enter Valid Number: ");
    }
}

fn parse_operation(op: &str) -> Option<Operation> {
    match op {
        "+" => Some(Operation::Add),
        "-" => Some(Operation::Subtract),
        "*" => Some(Operation::Multiply),
        "/" => Some(Operation::Divide),
        _ => None,
    }
}

fn quit_or_unwrap(opt: Option<f64>) -> f64 {
    match opt {
        Some(n) => n,
        None => {
            println!("Quitting Program");
            std::process::exit(0);
        }
    }
}

fn main() {
    welcome_prompt();

    loop {
        prompt("Enter The First Number: ");
        let first_number = quit_or_unwrap(get_parsed_number());

        prompt("Enter The Second Number: ");
        let second_number = quit_or_unwrap(get_parsed_number());

        prompt("Enter the operation: ");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Error Reading Line");

        let op = match parse_operation(operation.trim()) {
            Some(op) => op,
            None => {
                println!("Invalid Operation");
                return;
            }
        };

        if let Some(res) = op.apply(first_number, second_number) {
            println!("Result: {res:.2}");
        }
    }
}
