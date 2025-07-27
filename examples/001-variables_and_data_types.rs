// 001-variables_and_data_types.rs
// A comprehensive example showing variables, mutability, and all Rust data types

fn main() {
    println!("=== Variables and Data Types ===\n");

    // 1. Variables and Mutability
    println!("1. Variables and Mutability:");
    
    // Immutable variable
    let x = 5;
    println!("   Immutable x: {}", x);

    // Mutable variable
    let mut y = 10;
    println!("   Mutable y before: {}", y);
    y = 20;
    println!("   Mutable y after: {}", y);

    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("   Constant MAX_POINTS: {}", MAX_POINTS);

    // Shadowing
    let z = 2;
    let z = z * 2;
    println!("   Shadowed z: {}\n", z);

    // 2. Scalar Data Types
    println!("2. Scalar Data Types:");

    // Integers
    let small: i8 = -128;
    let medium: i32 = 123_456;
    let large: i64 = 9_223_372_036_854_775_807;
    let unsigned: u32 = 4_000_000;
    println!("   Integers: i8={}, i32={}, i64={}, u32={}", small, medium, large, unsigned);

    // Floating point
    let pi: f64 = 3.14159;
    let small_float: f32 = 2.5;
    println!("   Floats: f64={}, f32={}", pi, small_float);

    // Boolean
    let is_rust_awesome: bool = true;
    let is_learning_hard: bool = false;
    println!("   Booleans: awesome={}, hard={}", is_rust_awesome, is_learning_hard);

    // Character (Unicode)
    let letter: char = 'R';
    let emoji: char = '🦀';
    println!("   Characters: letter='{}', emoji='{}'\n", letter, emoji);

    // 3. Compound Data Types
    println!("3. Compound Data Types:");

    // Tuple
    let person: (String, i32, bool) = (String::from("Alice"), 30, true);
    let (name, age, is_employed) = person; // Destructuring
    println!("   Tuple - Person: {} is {} years old, employed: {}", name, age, is_employed);

    // Accessing tuple elements by index
    let coordinates = (3.0, 4.0);
    println!("   Tuple access: x={}, y={}", coordinates.0, coordinates.1);

    // Array (fixed size)
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let first = numbers[0];
    let length = numbers.len();
    println!("   Array: first={}, length={}, full={:?}", first, length, numbers);

    // Array with same value
    let zeros = [0; 3]; // [0, 0, 0]
    println!("   Array (same value): {:?}\n", zeros);

    // 4. String Types
    println!("4. String Types:");
    
    // String literals (str slice)
    let string_literal = "Hello, world!";
    println!("   String literal: {}", string_literal);
    
    // String (owned)
    let mut owned_string = String::from("Hello");
    owned_string.push_str(", Rust!");
    println!("   Owned String: {}", owned_string);
    
    // String slice
    let slice = &owned_string[0..5];
    println!("   String slice: {}\n", slice);

    // 5. Type Inference vs Explicit Types
    println!("5. Type Inference vs Explicit Types:");
    
    // Type inference
    let inferred = 42;
    println!("   Inferred type (i32): {}", inferred);
    
    // Explicit type annotation
    let explicit: f64 = 42.0;
    println!("   Explicit type (f64): {}", explicit);
    
    // Multiple possible types - need annotation
    let parsed: u32 = "42".parse().expect("Not a number!");
    println!("   Parsed with annotation: {}\n", parsed);

    // 6. Variable Scope and Lifetime
    println!("6. Variable Scope:");
    
    let outer = "outer scope";
    println!("   Outer variable: {}", outer);
    
    {
        let inner = "inner scope";
        println!("   Inner variable: {}", inner);
        println!("   Can access outer from inner: {}", outer);
    }
    // println!("   Cannot access inner here: {}", inner); // This would error
    
    println!("   Back to outer scope: {}\n", outer);

    // 7. Type Conversion
    println!("7. Type Conversion:");
    
    let integer = 65;
    let as_char = integer as u8 as char;
    let as_float = integer as f64;
    
    println!("   Integer {} as char: '{}', as float: {}", integer, as_char, as_float);
    
    // Safe conversion with try_into (would need use std::convert::TryInto)
    let safe_convert: u8 = 255;
    println!("   Safe u8: {}\n", safe_convert);

    // 8. Reference Types (preview for ownership lesson)
    println!("8. Reference Types (Preview):");
    
    let value = 42;
    let reference = &value;
    println!("   Value: {}, Reference to value: {}", value, reference);
    println!("   Reference points to address: {:p}\n", reference);

    println!("=== Summary ===");
    println!("✅ Learned about variables (immutable/mutable)");
    println!("✅ Explored constants and shadowing");
    println!("✅ Covered all scalar types (integers, floats, bool, char)");
    println!("✅ Understood compound types (tuples, arrays)");
    println!("✅ Worked with strings and string slices");
    println!("✅ Practiced type inference and explicit annotations");
    println!("✅ Explored scope and basic references\n");
    
    println!("Next: Move on to 002-functions.rs to learn about functions! 🚀");
}
