// Import the 'io' module from the standard library for input/output operations
use std::{io};

// Import the 'Rng' trait from the 'rand' crate to generate random numbers
use rand::Rng;

// Import the 'Ordering' enum for comparing values (Less, Greater, Equal)
use std::cmp::Ordering;

// The main function - this is where the program starts executing
fn main() {
    // Print a welcome message to the screen
    println!("Guess the number!");

    // Generate a random number between 1 and 100 (inclusive)
    // 'rand::rng()' creates a random number generator
    // 'random_range(1..=100)' generates a number from 1 to 100
    // The number is stored in 'secret_number' variable
    let secret_number = rand::rng().random_range(1..=100);

    // This line is commented out - it would reveal the secret number (useful for testing)
    // println!("The secret number is: {}", secret_number);

    // Ask the user to input their guess
    println!("Please input your guess.");

    // Create a new empty String to store the user's input
    // 'mut' makes the variable mutable (changeable)
    let mut guess = String::new();

    // Read a line of input from the user
    // 'io::stdin()' gets the standard input handle
    // '.read_line(&mut guess)' reads user input and stores it in 'guess'
    // '.expect()' handles potential errors - if reading fails, it shows this message
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    // Convert the String input to a number (u32 = unsigned 32-bit integer)
    // '.trim()' removes whitespace and newline characters from the input
    // '.parse()' converts the string to a number
    // If conversion fails (e.g., user typed "abc"), show error message
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    // Compare the user's guess with the secret number
    // '.cmp()' returns an Ordering enum: Less, Greater, or Equal
    match guess.cmp(&secret_number) {
        // If the guess is less than the secret number
        Ordering::Less => println!("Too small!"),

        // If the guess is greater than the secret number
        Ordering::Greater => println!("Too big!"),

        // If the guess equals the secret number
        Ordering::Equal => {
            println!("You win!");
        }
    }

    // This line is commented out - it would show what the user guessed
    // println!("You guessed: {}", guess);

    // Print a thank-you message
    println!("Thanks for playing!");

    // Print a goodbye message
    println!("Bye!")
}