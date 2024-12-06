extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to my guessing game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");   

    loop {
        println!("Guess the secret number");

        let mut user_guess = String::new();

        io::stdin()
            .read_line(&mut user_guess)
            .expect("Failed to read line");

        let user_guess: u32 = match user_guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };

        println!("You guessed {user_guess}");

        match user_guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
