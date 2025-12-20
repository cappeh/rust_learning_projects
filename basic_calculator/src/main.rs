use std::io;
use std::io::Write;

use crate::math::operations::{ add, subtract, multiply, divide };

pub mod math;

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide
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
    println!("Basic Calculator!");
    println!("Operations include '+,-,*,/'");

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

    match op {
        Operations::Add => {
            if let Some(res) = add(first_number, second_number) {
                println!("Result: {res}");
            }
        }
        Operations::Subtract => {
            if let Some(res) = subtract(first_number, second_number) {
                println!("Result: {res}");
            }
        }
        Operations::Multiply => {
            if let Some(res) = multiply(first_number, second_number) {
                println!("Result: {res}");
            }
        }
        Operations::Divide => {
            if let Some(res) = divide(first_number, second_number) {
                println!("Result: {res}");
            }
        }
        
    };
}
