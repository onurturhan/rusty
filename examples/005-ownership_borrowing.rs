// ownership_borrowing.rs
// Demonstrates Rust's unique ownership and borrowing system

fn main() {
    println!("=== Ownership and Borrowing ===");

    // 1. Ownership transfer (move)
    println!("1. Ownership Transfer:");
    let s1 = String::from("hello");
    let s2 = s1; // Ownership moved to s2
    // println!("{}", s1); // ERROR: value borrowed after move
    println!("   s2 now owns the value: {}\n", s2);

    // 2. Clone to avoid move
    println!("2. Clone to Keep Original:");
    let s3 = String::from("world");
    let s4 = s3.clone(); // Deep copy
    println!("   s3: {}, s4: {}\n", s3, s4);

    // 3. Borrowing with immutable references
    println!("3. Immutable Borrowing:");
    let s5 = String::from("borrowing");
    print_length(&s5); // Pass by reference
    println!("   After borrowing, s5 is still valid: {}\n", s5);

    // 4. Mutable references
    println!("4. Mutable Borrowing:");
    let mut number = 10;
    println!("   Before: {}", number);
    double_value(&mut number);
    println!("   After: {}\n", number);

    // 5. Multiple immutable references (OK)
    println!("5. Multiple Immutable References:");
    let text = String::from("shared");
    let r1 = &text;
    let r2 = &text;
    println!("   r1: {}, r2: {}\n", r1, r2);

    // 6. Scope and lifetimes
    println!("6. Reference Scopes:");
    let mut data = String::from("mutable");
    {
        let r1 = &data; // Immutable borrow starts
        println!("   Immutable reference: {}", r1);
    } // r1 goes out of scope here
    
    let r2 = &mut data; // Mutable borrow - OK now
    r2.push_str(" data");
    println!("   Mutable reference: {}\n", r2);

    // 7. String slices
    println!("7. String Slices:");
    let sentence = String::from("Hello Rust world");
    let hello = &sentence[0..5];
    let world = &sentence[11..16];
    println!("   Full: {}", sentence);
    println!("   Slice 1: {}, Slice 2: {}\n", hello, world);

    // 8. Function that returns ownership
    println!("8. Returning Ownership:");
    let new_string = create_string();
    println!("   Created: {}\n", new_string);

    println!("=== End of Ownership Examples ===");
}

fn print_length(s: &String) {
    println!("   The length of '{}' is {}.", s, s.len());
}

fn double_value(value: &mut i32) {
    *value *= 2;
}

// Function that creates and returns a String
fn create_string() -> String {
    String::from("created in function")
}
