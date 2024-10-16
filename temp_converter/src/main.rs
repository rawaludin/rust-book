use std::{io, u32};
fn fahrenheit_to_celcius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn celcius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    loop {
        println!("Temperature Converter");
        println!("1. Fahrenheit to Celcius");
        println!("2. Celcius to Fahrenheit");

        let choice = loop {
            println!("Choose an option (1 or 2)");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<u32>() {
                Ok(num) if num == 1 || num == 2 => break num,
                _ => println!("Invalid option. Please enter 1 or 2."),
            }
        };

        match choice {
            1 => {
                println!("Enter temperature in Fahrenheit:");
                let temp: f64 = loop {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    match input.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Invalid temperature. Please enter valid number."),
                    }
                };
                let celcius = fahrenheit_to_celcius(temp);
                println!("{:.2} Fahrenheit is {:.2} Celcius", temp, celcius);
            }

            2 => {
                println!("Enter temperature in Celcius:");
                let temp: f64 = loop {
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("Failed to read line");
                    match input.trim().parse() {
                        Ok(num) => break num,
                        Err(_) => println!("Invalid temperature. Please enter valid number."),
                    }
                };
                let fahrenheit = celcius_to_fahrenheit(temp);
                println!("{:.2} Celcius is {:.2} Fahrenheit", temp, fahrenheit);
            }

            _ => unreachable!(),
        }

        println!("Do you want to convert another temperature? (y/n)");
        let mut again = String::new();
        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read line");
        if again.trim().to_lowercase() != "y" {
            break;
        }
    }
}
