# Rust Cheatsheet 🦀

## Basic Syntax

### Variables
```rust
let x = 5;                    // Immutable
let mut y = 10;               // Mutable
const MAX: u32 = 100;         // Constant
let x = x * 2;                // Shadowing
```

### Data Types
```rust
// Integers
let num: i32 = 42;            // 32-bit signed
let big: u64 = 123;           // 64-bit unsigned

// Floats
let pi: f64 = 3.14159;        // 64-bit float

// Others
let flag: bool = true;        // Boolean
let letter: char = 'A';       // Character
let text: &str = "Hello";     // String slice
let owned: String = String::from("World"); // Owned string
```

### Functions
```rust
fn function_name(param: i32) -> i32 {
    param * 2  // Expression return (no semicolon)
}

fn no_return(x: i32) {
    println!("{}", x);  // Statement (with semicolon)
}
```

### Control Flow
```rust
// If/else
if condition {
    // code
} else if other_condition {
    // code
} else {
    // code
}

// Match (like switch)
match value {
    1 => println!("One"),
    2 | 3 => println!("Two or Three"),
    4..=10 => println!("Four to Ten"),
    _ => println!("Something else"),
}

// Loops
loop {
    break;  // Infinite loop
}

while condition {
    // code
}

for item in collection {
    // code
}

for i in 0..5 {  // Range 0 to 4
    // code
}
```

### Ownership & Borrowing
```rust
let s1 = String::from("hello");
let s2 = s1;              // s1 is moved, no longer valid

let s3 = String::from("world");
let s4 = s3.clone();      // Deep copy, both valid

let s5 = String::from("rust");
let len = calculate_length(&s5);  // Borrow (s5 still valid)

fn calculate_length(s: &String) -> usize {
    s.len()  // s is a reference
}
```

### Structs
```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }
    
    fn greet(&self) {
        println!("Hi, I'm {}", self.name);
    }
}

let person = Person::new(String::from("Alice"), 30);
person.greet();
```

### Enums
```rust
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

let color = Color::RGB(255, 0, 0);

match color {
    Color::Red => println!("Red!"),
    Color::RGB(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
    _ => println!("Other color"),
}
```

### Error Handling
```rust
// Option<T> - for nullable values
let maybe_number: Option<i32> = Some(5);
match maybe_number {
    Some(n) => println!("Number: {}", n),
    None => println!("No number"),
}

// Result<T, E> - for error handling
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

match divide(10.0, 2.0) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}
```

### Collections
```rust
// Vector
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
let vec2 = vec![1, 2, 3];

// HashMap
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("key", "value");
```

### Common Macros
```rust
println!("Hello, {}!", name);      // Print with formatting
print!("No newline");              // Print without newline
eprintln!("Error message");        // Print to stderr

vec![1, 2, 3];                     // Create vector
format!("Formatted {}", string);   // Format string

panic!("Something went wrong!");    // Panic (crash program)
```

### Useful Attributes
```rust
#[derive(Debug)]                   // Auto-implement Debug trait
#[allow(dead_code)]               // Suppress warnings
#[cfg(test)]                      // Compile only for tests
```

## Cargo Commands
```bash
cargo new project_name            # Create new project
cargo init                        # Initialize in current dir
cargo build                       # Build project
cargo run                         # Build and run
cargo check                       # Check for errors (faster)
cargo test                        # Run tests
cargo doc --open                  # Generate and open docs
cargo clean                       # Clean build artifacts
```

## Common Patterns

### Iterator Methods
```rust
let numbers = vec![1, 2, 3, 4, 5];

// Map, filter, collect
let doubled: Vec<i32> = numbers.iter()
    .map(|x| x * 2)
    .filter(|&x| x > 4)
    .collect();

// Find
let found = numbers.iter().find(|&&x| x > 3);

// Fold/reduce
let sum = numbers.iter().fold(0, |acc, x| acc + x);
```

### String Manipulation
```rust
let s = String::from("hello");
let s2 = s + " world";                    // Consumes s
let s3 = format!("{} {}", "hello", "world"); // Doesn't consume

let slice = &s3[0..5];                    // String slice
```

This cheatsheet covers the most commonly used Rust syntax. Keep it handy while learning! 🦀
