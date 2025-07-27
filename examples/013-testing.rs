// 013-testing.rs
// This example demonstrates testing in Rust
// Run with: rustc --test examples/013-testing.rs -o 013-testing && ./013-testing
// Or for individual functions: rustc examples/013-testing.rs -o 013-testing && ./013-testing

fn main() {
    println!("=== Rust Testing Examples ===\n");

    // Testing concepts
    testing_concepts();
    
    // Unit testing examples  
    unit_testing_examples();
    
    // Integration testing info
    integration_testing_info();
    
    // Test organization
    test_organization_info();
    
    // Testing best practices
    testing_best_practices();
}

fn testing_concepts() {
    println!("1. Testing Concepts in Rust");
    
    println!("Rust has built-in testing framework with:");
    println!("  - #[test] attribute to mark test functions");
    println!("  - assert! macros for assertions");
    println!("  - cargo test command to run tests");
    println!("  - Automatic test discovery");
    println!("  - Parallel test execution by default");
    println!();
    
    println!("Types of tests:");
    println!("  - Unit tests: Test individual functions/modules");
    println!("  - Integration tests: Test public API");
    println!("  - Documentation tests: Examples in doc comments");
    println!();
}

fn unit_testing_examples() {
    println!("2. Unit Testing Examples");
    
    // Example functions to test
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Cannot divide by zero"))
        } else {
            Ok(a / b)
        }
    }
    
    #[derive(Debug, PartialEq)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        fn area(&self) -> u32 {
            self.width * self.height
        }
        
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    // Demonstrate the functions work
    println!("Function examples:");
    println!("add(2, 3) = {}", add(2, 3));
    
    match divide(10.0, 2.0) {
        Ok(result) => println!("divide(10.0, 2.0) = {}", result),
        Err(e) => println!("Error: {}", e),
    }
    
    let rect = Rectangle::new(8, 7);
    println!("Rectangle area: {}", rect.area());
    
    println!();
}

fn integration_testing_info() {
    println!("3. Integration Testing");
    
    println!("Integration tests go in tests/ directory:");
    println!("  project/");
    println!("  ├── src/");
    println!("  │   └── lib.rs");
    println!("  └── tests/");
    println!("      ├── integration_test.rs");
    println!("      └── common/");
    println!("          └── mod.rs");
    println!();
    
    println!("Example integration test structure:");
    println!("  // tests/integration_test.rs");
    println!("  use my_crate;");
    println!("  ");
    println!("  #[test]");
    println!("  fn test_public_api() {{");
    println!("      assert_eq!(my_crate::add(2, 3), 5);");
    println!("  }}");
    println!();
}

fn test_organization_info() {
    println!("4. Test Organization");
    
    println!("Common test organization patterns:");
    println!("  1. Tests module at end of file:");
    println!("     #[cfg(test)]");
    println!("     mod tests {{");
    println!("         use super::*;");
    println!("         #[test]");
    println!("         fn test_function() {{ ... }}");
    println!("     }}");
    println!();
    
    println!("  2. Separate test files in tests/ directory");
    println!("  3. Documentation tests in doc comments");
    println!();
    
    println!("Test attributes:");
    println!("  #[test] - Mark function as test");
    println!("  #[ignore] - Skip test by default");
    println!("  #[should_panic] - Test should panic");
    println!("  #[cfg(test)] - Only compile in test mode");
    println!();
}

fn testing_best_practices() {
    println!("5. Testing Best Practices");
    
    println!("Best practices for Rust testing:");
    println!("  ✓ Write tests for both success and failure cases");
    println!("  ✓ Use descriptive test names");
    println!("  ✓ Test one thing per test function");
    println!("  ✓ Use assert_eq! and assert_ne! for better error messages");
    println!("  ✓ Test edge cases and boundary conditions");
    println!("  ✓ Use Result<(), Box<dyn Error>> for tests that can fail");
    println!("  ✓ Group related tests in modules");
    println!("  ✓ Use #[cfg(test)] to keep test code separate");
    println!();
    
    println!("Common assertion macros:");
    println!("  assert!(condition) - Assert condition is true");
    println!("  assert_eq!(left, right) - Assert equality");
    println!("  assert_ne!(left, right) - Assert inequality");
    println!("  panic!(\"message\") - Panic with message");
    println!();
    
    println!("Running tests:");
    println!("  cargo test - Run all tests");
    println!("  cargo test test_name - Run specific test");
    println!("  cargo test --ignored - Run ignored tests");
    println!("  cargo test -- --nocapture - Show println! output");
    println!();
    
    println!("\n=== Testing Complete! ===");
}

// Example of how tests would be structured in a real module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        fn add(a: i32, b: i32) -> i32 { a + b }
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative_numbers() {
        fn add(a: i32, b: i32) -> i32 { a + b }
        assert_eq!(add(-2, -3), -5);
    }

    #[test]
    fn test_add_zero() {
        fn add(a: i32, b: i32) -> i32 { a + b }
        assert_eq!(add(5, 0), 5);
        assert_eq!(add(0, 5), 5);
    }

    #[test]
    fn test_divide_success() {
        fn divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 { Err(String::from("Cannot divide by zero")) } else { Ok(a / b) }
        }
        
        match divide(10.0, 2.0) {
            Ok(result) => assert_eq!(result, 5.0),
            Err(_) => panic!("Expected Ok, got Err"),
        }
    }

    #[test]
    fn test_divide_by_zero() {
        fn divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 { Err(String::from("Cannot divide by zero")) } else { Ok(a / b) }
        }
        
        match divide(10.0, 0.0) {
            Ok(_) => panic!("Expected Err, got Ok"),
            Err(message) => assert_eq!(message, "Cannot divide by zero"),
        }
    }

    #[test]
    fn test_rectangle_area() {
        #[derive(Debug, PartialEq)]
        struct Rectangle { width: u32, height: u32 }
        impl Rectangle {
            fn new(width: u32, height: u32) -> Rectangle { Rectangle { width, height } }
            fn area(&self) -> u32 { self.width * self.height }
        }
        
        let rect = Rectangle::new(8, 7);
        assert_eq!(rect.area(), 56);
    }

    #[test]
    fn test_rectangle_can_hold() {
        #[derive(Debug, PartialEq)]
        struct Rectangle { width: u32, height: u32 }
        impl Rectangle {
            fn new(width: u32, height: u32) -> Rectangle { Rectangle { width, height } }
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
        
        let larger = Rectangle::new(8, 7);
        let smaller = Rectangle::new(5, 1);
        let same_size = Rectangle::new(8, 7);

        assert!(larger.can_hold(&smaller));
        assert!(!smaller.can_hold(&larger));
        assert!(!larger.can_hold(&same_size));
    }

    #[test]
    #[should_panic(expected = "Values must be positive")]
    fn test_should_panic_example() {
        fn validate_positive(value: i32) {
            if value <= 0 {
                panic!("Values must be positive");
            }
        }
        
        validate_positive(-1); // This should panic
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // This test is ignored by default
        // Run with: cargo test -- --ignored
        println!("This test would take a long time to run");
    }

    #[test]
    fn test_with_result() -> Result<(), Box<dyn std::error::Error>> {
        fn might_fail(should_fail: bool) -> Result<i32, &'static str> {
            if should_fail { Err("Something went wrong") } else { Ok(42) }
        }
        
        let result = might_fail(false)?;
        assert_eq!(result, 42);
        
        Ok(())
    }
}

// Example of testing private functions
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        internal_add(a, b)
    }
    
    fn internal_add(a: i32, b: i32) -> i32 {
        a + b
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_public_add() {
            assert_eq!(add(2, 3), 5);
        }
        
        #[test]
        fn test_internal_add() {
            // Can test private functions from within the same module
            assert_eq!(internal_add(2, 3), 5);
        }
    }
}

// Example of custom assertion helper
#[cfg(test)]
fn assert_approx_equal(a: f64, b: f64, tolerance: f64) {
    assert!((a - b).abs() < tolerance, "Values {} and {} are not approximately equal", a, b);
}

#[cfg(test)]
mod floating_point_tests {
    use super::*;
    
    #[test]
    fn test_floating_point_arithmetic() {
        let result = 0.1 + 0.2;
        // Don't use assert_eq! with floats due to precision issues
        assert_approx_equal(result, 0.3, 1e-10);
    }
}
