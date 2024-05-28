use std::io;
// import standard input/output library
use rand::Rng;
// import random number generator
use std::cmp::Ordering;
// import standard library to compare values

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // create new variable 'secret_number' and set it to randon number between 1 and 100

    loop { // loop to allow multiple guesses
        println!("Please input your guess.");

        let mut guess = String::new();
        // initalize new mutable variable 'guess' as an empty string

        io::stdin()
            .read_line(&mut guess)  // read user input and store it to 'guess' variable via reference
            .expect("Failed to read line"); // error checking

        // convert string to integer to later match with secret_number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // check for integer
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // implement code to compare value of guess and secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // exit program after correct guess
            }
        }
    }
}