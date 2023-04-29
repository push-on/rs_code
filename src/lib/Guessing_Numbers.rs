use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number!: ");
        let input = get_input();

        match input.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".blue()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}", "You win!".green());
                break;
            }
        }
    }
}

fn get_input() -> u32 {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}", "Invalid input, please enter a number.".purple());
                continue;
            }
        };
        return input;
    }
}
