use std::io;
use std::io::Write;

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn get_parsed_number() -> u32 {
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Error Reading Stdin");
    let number: u32 = number.trim().parse().expect("Invalid Number");
    number
}

fn main() {
    println!("Basic Calculator!");
    println!("Operations include '+,-,*,/'");

    print!("Enter the first number: ");
    io::stdout().flush().unwrap();
    let first_number = get_parsed_number();
    io::stdout().flush().unwrap();


    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let second_number = get_parsed_number();

    print!("Enter the operation: ");
    io::stdout().flush().unwrap();
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

    let result = match op {
        Operations::Add => first_number + second_number,
        Operations::Subtract => first_number - second_number,
        Operations::Multiply => first_number * second_number,
        Operations::Divide => first_number / second_number,
    };

    println!("Result: {result}");
}
