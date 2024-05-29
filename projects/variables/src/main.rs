fn main() {
    let mut x = 5; // x must be mutable to reassign the value to 6 below
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // example of constant
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // example of variable shadowing

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // using shadowing to change type of variable

    let spaces = "    ";
    println!("{spaces}");
    let spaces = spaces.len();
    println!("{spaces}");

    // this would not work
    // let mut spaces = "    ";
    // spaces = spaces.len();
}
