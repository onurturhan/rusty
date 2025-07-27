// data_types.rs
// Rust data types examples

fn main() {
    // Scalar Types

    // Integers
    let small: i8 = -128;
    let medium: i32 = 123_456;
    let large: i64 = 9_223_372_036_854_775_807;
    let unsigned: u32 = 4_000_000;

    // Floating point
    let pi: f64 = 3.14159;
    let small_float: f32 = 2.5;

    // Boolean
    let is_rust_awesome: bool = true;
    let is_learning_hard: bool = false;

    // Character (Unicode)
    let letter: char = 'R';
    let emoji: char = '🦀';

    println!("Integer examples: {}, {}, {}, {}", small, medium, large, unsigned);
    println!("Float examples: {}, {}", pi, small_float);
    println!("Boolean examples: {}, {}", is_rust_awesome, is_learning_hard);
    println!("Character examples: {}, {}", letter, emoji);

    // Compound Types

    // Tuple
    let person: (String, i32, bool) = (String::from("Alice"), 30, true);
    let (name, age, is_employed) = person; // Destructuring
    println!("Person: {} is {} years old, employed: {}", name, age, is_employed);

    // Array (fixed size)
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let first = numbers[0];
    let length = numbers.len();
    println!("First number: {}, Array length: {}", first, length);

    // Array with same value
    let zeros = [0; 3]; // [0, 0, 0]
    println!("Zeros array: {:?}", zeros);
}
