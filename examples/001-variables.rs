// variables.rs
// A simple Rust example showing variables and data types

fn main() {
    // Immutable variable
    let x = 5;
    println!("The value of x is: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("The value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("Max points are: {}", MAX_POINTS);

    // Shadowing
    let z = 2;
    let z = z * 2;
    println!("The value of z using shadowing is: {}", z);
}

