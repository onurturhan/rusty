# Rust Learning Guide

Welcome to Rust! This guide will take you from beginner to intermediate level with hands-on examples.

## 🦀 What is Rust?

Rust is a systems programming language focused on:
- **Memory Safety**: No null pointer dereferences, buffer overflows, or memory leaks
- **Performance**: Zero-cost abstractions, no garbage collector
- **Concurrency**: Safe concurrent programming
- **Cross-platform**: Runs on many platforms and architectures

## 📚 Learning Path

### Phase 1: Basics (Days 1-7)
1. Variables and Data Types
2. Functions
3. Control Flow (if/else, loops)
4. Ownership and Borrowing (Rust's unique feature!)
5. Structs and Enums
6. Pattern Matching
7. Error Handling

### Phase 2: Intermediate (Days 8-14)
1. Collections (Vec, HashMap, etc.)
2. Generics and Traits
3. Lifetimes
4. Modules and Crates
5. File I/O
6. Testing
7. Command Line Programs

### Phase 3: Advanced (Days 15+)
1. Concurrency and Threads
2. Async Programming
3. Unsafe Rust
4. Macros
5. Web Development
6. System Programming

## 🚀 Getting Started

### Essential Commands
```bash
# Create a new project
cargo new project_name

# Build the project
cargo build

# Run the project
cargo run

# Check for errors without building
cargo check

# Run tests
cargo test

# Build optimized version
cargo build --release
```

### Project Structure
```
my_project/
├── Cargo.toml          # Project configuration
├── src/
│   ├── main.rs         # Main entry point
│   └── lib.rs          # Library code (optional)
├── tests/              # Integration tests
└── target/             # Compiled output (auto-generated)
```

## 📖 Learning Resources

### Official Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/) - The official book
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by examples
- [The Cargo Book](https://doc.rust-lang.org/cargo/) - Package manager guide

### Interactive Learning
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises
- [Rust Playground](https://play.rust-lang.org/) - Online Rust compiler

### Practice Projects
1. CLI Calculator
2. File Organizer
3. Web Scraper
4. Chat Server
5. Game (like Guessing Game, Tic-tac-toe)

## 🎯 Next Steps

1. Complete the exercises in the `examples/` directory
2. Read "The Rust Programming Language" book chapters 1-10
3. Try the Rustlings exercises
4. Build a small project of your choice

## 💡 Key Rust Concepts to Remember

1. **Ownership**: Each value has a single owner
2. **Borrowing**: You can borrow references to values
3. **Lifetimes**: How long references are valid
4. **Pattern Matching**: Powerful `match` expressions
5. **Error Handling**: `Result<T, E>` and `Option<T>` types
6. **Traits**: Similar to interfaces in other languages

## 🔧 Development Tips

### Compiler is Your Friend
- Read error messages carefully - they're very helpful!
- Use `cargo check` for faster error checking
- Don't fight the borrow checker - learn from it

### Best Practices
- Start small and build up complexity
- Use `cargo fmt` to format your code
- Use `cargo clippy` for additional lints
- Write tests as you develop

### Common Patterns

#### Error Handling
```rust
// Using Result
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

// Using Option
fn find_word(text: &str, word: &str) -> Option<usize> {
    text.find(word)
}
```

#### Iterator Patterns
```rust
let numbers = vec![1, 2, 3, 4, 5];

// Filter and collect
let evens: Vec<i32> = numbers
    .iter()
    .filter(|&x| x % 2 == 0)
    .copied()
    .collect();

// Map and sum
let sum: i32 = numbers
    .iter()
    .map(|x| x * x)
    .sum();
```

## 🏗️ Project Ideas by Difficulty

### Beginner
- Temperature converter
- Simple calculator
- Word counter
- File reader
- Basic CLI tools

### Intermediate
- Web server with Actix or Axum
- Database integration
- JSON API
- Command-line application with clap
- Simple game

### Advanced
- Async web crawler
- Database implementation
- Compiler or interpreter
- Network protocol implementation
- Performance-critical system tools

Happy learning! 🦀
