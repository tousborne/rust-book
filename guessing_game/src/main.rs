/// Chapter 2:
/// This chapter covers a few rust foundational concepts such as libraries, loops,
/// mutability, using stdin and stdout, and matches while building a guessing game.
///
/// The Guessing Game takes user input and compares it to a randomly generated number.
/// If the user guesses correctly a success message is printed, otherwise the program
/// loops back to getting user input.

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

const MIN: u32 = 0;
const MAX: u32 = 100;

// Generate a secret number and compare guesses.
fn main() {
    // Randomly generated secret number between MIN and MAX.
    let secret = rand::thread_rng()
        .gen_range(MIN, MAX);

    loop {
        // The print! macro usually requires that stdout be flushed.
        print!("Guess a number ({} - {}): ", MIN, MAX-1);
        io::stdout().flush().unwrap();

        // Read the guess from stdin.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input.");

        // Parse the guess into a number, checking for errors.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Compare the guess to the secret using and ordering comparison.
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You got it!");
                break;
            },

            // Catch everything else.
            _ => println!("What type of asshole guesses {}?!", guess),
        }
    }
}
