mod fibonacci;

use fibonacci::fibonacci;
use std::io::{self, Write};

fn main() {
    loop {
        print!("Enter the nth Fibonacci number to generate (or 'q' to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        if input.to_lowercase() == "q" {
            println!("Goodbye!");
            break;
        }

        match input.parse::<u32>() {
            Ok(n) if n >= 1 => {
                let result = fibonacci(n);
                println!("The {}th Fibonacci number is: {}", n, result);
            }
            Ok(_) => {
                println!("Invalid input. Please enter a positive integer greater than or equal to 1, or 'q' to quit.");
            }
            Err(_) => {
                println!("Invalid input. Please enter a positive integer greater than or equal to 1, or 'q' to quit.");
            }
        }
    }
}
