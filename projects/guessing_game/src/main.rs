use std::io;
// import standard input/output library

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();
    // initalize new mutable variable 'guess' as an empty string

    io::stdin()
        .read_line(&mut guess)  // read user input and store it to 'guess' variable via reference
        .expect("Failed to read line"); // error checking

    println!("You guessed: {guess}");
}