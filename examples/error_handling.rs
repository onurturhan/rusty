// error_handling.rs
// Demonstrates Rust's error handling with Result and Option

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== Error Handling ===\n");

    // 1. Option - handling nullable values
    println!("1. Option Type:");
    let numbers = vec![1, 2, 3, 4, 5];
    
    match find_number(&numbers, 3) {
        Some(index) => println!("   Found 3 at index: {}", index),
        None => println!("   Number 3 not found"),
    }
    
    match find_number(&numbers, 10) {
        Some(index) => println!("   Found 10 at index: {}", index),
        None => println!("   Number 10 not found"),
    }
    println!();

    // 2. Result - handling operations that can fail
    println!("2. Result Type:");
    match divide(10.0, 2.0) {
        Ok(result) => println!("   10 / 2 = {}", result),
        Err(error) => println!("   Error: {}", error),
    }
    
    match divide(10.0, 0.0) {
        Ok(result) => println!("   10 / 0 = {}", result),
        Err(error) => println!("   Error: {}", error),
    }
    println!();

    // 3. Unwrap and expect (use carefully!)
    println!("3. Unwrap and Expect:");
    let some_value = Some(42);
    let unwrapped = some_value.unwrap(); // Panics if None
    println!("   Unwrapped value: {}", unwrapped);
    
    let expected = Some(100).expect("This should have a value");
    println!("   Expected value: {}", expected);
    println!("   Note: unwrap() and expect() will panic on None/Err!\n");

    // 4. Unwrap_or and unwrap_or_else
    println!("4. Safe Unwrapping:");
    let maybe_number: Option<i32> = None;
    let default_value = maybe_number.unwrap_or(42);
    println!("   Default value: {}", default_value);
    
    let computed_default = maybe_number.unwrap_or_else(|| {
        println!("   Computing default value...");
        100
    });
    println!("   Computed default: {}", computed_default);
    println!();

    // 5. The ? operator for error propagation
    println!("5. Error Propagation with ?:");
    match read_file_contents("test.txt") {
        Ok(contents) => println!("   File contents: {}", contents),
        Err(error) => println!("   Failed to read file: {}", error),
    }
    println!();

    // 6. Chaining Option and Result operations
    println!("6. Chaining Operations:");
    let numbers_str = vec!["1", "2", "not_a_number", "4"];
    
    for num_str in numbers_str {
        let result = parse_and_double(num_str);
        match result {
            Ok(doubled) => println!("   '{}' -> {}", num_str, doubled),
            Err(error) => println!("   '{}' -> Error: {}", num_str, error),
        }
    }
    println!();

    // 7. Custom error types
    println!("7. Custom Errors:");
    match validate_age(25) {
        Ok(age) => println!("   Valid age: {}", age),
        Err(error) => println!("   Error: {}", error),
    }
    
    match validate_age(-5) {
        Ok(age) => println!("   Valid age: {}", age),
        Err(error) => println!("   Error: {}", error),
    }
    
    match validate_age(150) {
        Ok(age) => println!("   Valid age: {}", age),
        Err(error) => println!("   Error: {}", error),
    }
    println!();

    // 8. Converting between Option and Result
    println!("8. Converting Between Option and Result:");
    let maybe_number = Some(42);
    let result_number: Result<i32, &str> = maybe_number.ok_or("No number found");
    println!("   Option to Result: {:?}", result_number);
    
    let back_to_option: Option<i32> = result_number.ok();
    println!("   Result to Option: {:?}", back_to_option);
    println!();

    println!("=== End of Error Handling ===");
}

// Function that returns Option
fn find_number(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, &number) in numbers.iter().enumerate() {
        if number == target {
            return Some(index);
        }
    }
    None
}

// Function that returns Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Function using the ? operator for error propagation
fn read_file_contents(filename: &str) -> Result<String, io::Error> {
    let mut file = File::open(filename)?; // ? propagates the error if file opening fails
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // ? propagates the error if reading fails
    Ok(contents)
}

// Function that chains operations
fn parse_and_double(s: &str) -> Result<i32, String> {
    let number: i32 = s.parse()
        .map_err(|_| format!("'{}' is not a valid number", s))?;
    Ok(number * 2)
}

// Custom error type example
#[derive(Debug)]
enum AgeError {
    Negative,
    TooOld,
}

impl std::fmt::Display for AgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AgeError::Negative => write!(f, "Age cannot be negative"),
            AgeError::TooOld => write!(f, "Age cannot be over 120"),
        }
    }
}

impl std::error::Error for AgeError {}

fn validate_age(age: i32) -> Result<u32, AgeError> {
    if age < 0 {
        Err(AgeError::Negative)
    } else if age > 120 {
        Err(AgeError::TooOld)
    } else {
        Ok(age as u32)
    }
}
