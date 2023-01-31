// Bring in standard library and external crate for usage
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// Main function is the entry point for the program
fn main() {
    // Macro that prints a string to the screen
    println!("Guess the number");

    // Gives us the particular random number generator
    // And generates a random number in the given range (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Infinite loop
    loop {
        println!("Input your guess");

        // Creating a mutable variable that is an empty string
        let mut guess = String::new();

        // Function to handle user inpput
        io::stdin()
            // Gets input from user and returns a "Result" value
            // "&" indicates a reference to a certain variable rather than a copy
            .read_line(&mut guess)
            // Handles the error by causing the program to crash
            .expect("Failed to read line");

        // Trim any whitespaces and then convert the string into an unsigned 32-bit int
        // Handle error by using a "match" expression to decide what to do with "Result" value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // "{}" is a placeholder for a variable to go inside of it
        println!("You guessed: {guess}");

        // "Match" expression to decide what to do next based on a certain
        // "Ordering" variant from comparing the guess to the secret number
        match guess.cmp(&secret_number) {
            // "Arms" of the match experssion that are patterns to match against
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
