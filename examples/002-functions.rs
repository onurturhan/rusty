// functions.rs
// Rust functions examples

fn main() {
    println!("Hello from main function!");
    
    // Function with no parameters
    greet();
    
    // Function with parameters
    introduce("Alice", 25);
    
    // Function with return value
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    // Function with expression return (no semicolon)
    let product = multiply(4, 7);
    println!("4 * 7 = {}", product);
    
    // Function returning tuple
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} remainder {}", quotient, remainder);
}

// Function with no parameters or return value
fn greet() {
    println!("Hello, Rust learner!");
}

// Function with parameters
fn introduce(name: &str, age: u32) {
    println!("Hi, I'm {} and I'm {} years old.", name, age);
}

// Function with return value (explicit return)
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

// Function with expression return (no semicolon at end)
fn multiply(a: i32, b: i32) -> i32 {
    a * b  // No semicolon = expression that returns value
}

// Function returning multiple values (tuple)
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}
