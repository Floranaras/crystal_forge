use std::io::{self, Write};

pub fn get_integer(prompt: &str, min: i32, max: i32) -> i32 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().parse::<i32>() {
            Ok(value) if value >= min && value <= max => return value,
            Ok(_) => println!("Invalid input. Please enter a value between {} and {}.", min, max),
            Err(_) => println!("Invalid input. Please enter a valid number."),
        }
    }
}

pub fn get_yes_no(prompt: &str) -> bool {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Invalid choice. Please enter 'y' or 'n'."),
        }
    }
}
