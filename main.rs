use std::io::{self, Write};

fn main() {
    clear_console();
    println!("Welcome to the Rust Calculator!");

    loop {
        println!("\nChoose an operation:");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("0. Quit");

        print!("\nYour choice: ");
        io::stdout().flush().unwrap();

        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Failed to read line");

        match operation.trim().parse::<u32>() {
            Ok(1) => perform_operation('+', "addition"),
            Ok(2) => perform_operation('-', "subtraction"),
            Ok(3) => perform_operation('*', "multiplication"),
            Ok(4) => perform_operation('/', "division"),
            Ok(0) => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid operation. Please choose a number between 0 and 4."),
        }
    }
}

fn perform_operation(operator: char, operation_str: &str) {
    println!("You chose {}.", operation_str);
    println!("Enter the first number:");
    let num1: f64 = get_input_number();

    println!("Enter the second number:");
    let num2: f64 = get_input_number();

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num2,
        '*' => num1 * num2,
        '/' => {
            if num2 == 0.0 {
                println!("Error: Division by zero!");
                return;
            } else {
                num1 / num2
            }
        }
        _ => {
            println!("Invalid operator.");
            return;
        }
    };

    println!("Result: {}", result);
}

fn get_input_number() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            get_input_number()
        }
    }
}

fn clear_console() {
    if cfg!(windows) {
        let _ = std::process::Command::new("cmd").arg("/c").arg("cls").status();
    } else {
        let _ = std::process::Command::new("clear").status();
    }
}
