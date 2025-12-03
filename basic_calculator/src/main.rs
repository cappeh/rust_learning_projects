use std::io;

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn main() {
    println!("Basic Calculator!");
    println!("Operations include '+,-,*,/'");

    println!("Enter the first number:");
    let mut first_number = String::new();
    io::stdin().read_line(&mut first_number).expect("error reading line");
    let first_number: u32 = first_number.trim().parse().expect("not a number");


    println!("Enter the second number:");
    let mut second_number = String::new();
    io::stdin().read_line(&mut second_number).expect("error reading line");
    let second_number: u32 = second_number.trim().parse().expect("not a number");

    println!("enter the operation:");
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
