use std::io;
// import standard input/output library
use rand::Rng;
// import random number generator

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // create new variable 'secret_number' and set it to randon number between 1 and 100

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();
    // initalize new mutable variable 'guess' as an empty string

    io::stdin()
        .read_line(&mut guess)  // read user input and store it to 'guess' variable via reference
        .expect("Failed to read line"); // error checking

    println!("You guessed: {guess}");
}