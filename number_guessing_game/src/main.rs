use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Number Guessing Game!");
    println!("The Secret Number is between 1 and 100");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Guess a number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("please enter a valid number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Low"),
            Ordering::Greater => println!("Too High"),
            Ordering::Equal => {
                println!("You Guessed Correctly");
                break;
            }
            
        }
    }
}
