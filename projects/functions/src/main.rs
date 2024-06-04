fn main() {
    println!("Hello, world!");

    another_function();

    parameter_example(5);

    print_labeled_measurement(5, 'h');

    // example of an expression
    let y = {
        let x = 3;
        x + 1 // note that expression does not have ending semicolon
    };
    println!("The value of y is: {y}");  // prints '4'

    let x = five();
    println!("The value of x is: {x}");

    let z = plus_one(5);
    println!("The value of z is: {z}");
}

fn another_function() {
    println!("Another function.");
}

fn parameter_example(x: i32) { // must set type for parameter
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) { // example of multiple parameters
    println!("The measurement is: {value}{unit_label}");
}

// example of function with return value
fn five() -> i32 {
    5
}

// example of function with return value that also has parameter
fn plus_one(x: i32) -> i32 {
    x + 1
}