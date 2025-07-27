// 011-modules-crates.rs  
// This example demonstrates Rust's module system and crate organization
// Run with: rustc examples/011-modules-crates.rs -o 011-modules-crates && ./011-modules-crates

fn main() {
    println!("=== Rust Modules and Crates Examples ===\n");

    // Module basics
    module_basics();
    
    // Privacy and visibility
    privacy_examples();
    
    // Use statements and paths
    use_statements();
    
    // Creating and using modules
    working_with_modules();
    
    // External crates (conceptual)
    external_crates_info();
}

fn module_basics() {
    println!("1. Module Basics");
    
    // Inline module definition
    mod greeting {
        pub fn hello() {
            println!("Hello from the greeting module!");
        }
        
        pub fn goodbye() {
            println!("Goodbye from the greeting module!");
        }
        
        // Private function - not accessible outside module
        fn private_function() {
            println!("This is private");
        }
        
        pub mod nested {
            pub fn nested_hello() {
                println!("Hello from nested module!");
            }
        }
    }
    
    // Using functions from modules
    greeting::hello();
    greeting::goodbye();
    greeting::nested::nested_hello();
    
    // greeting::private_function(); // This would cause a compile error
    
    println!();
}

fn privacy_examples() {
    println!("2. Privacy and Visibility");
    
    mod restaurant {
        pub mod front_of_house {
            pub mod hosting {
                pub fn add_to_waitlist() {
                    println!("Adding customer to waitlist");
                }
                
                fn seat_at_table() {
                    println!("Seating customer at table");
                }
            }
            
            pub mod serving {
                pub fn take_order() {
                    println!("Taking customer order");
                }
                
                pub fn serve_order() {
                    println!("Serving order to customer");
                }
                
                fn take_payment() {
                    println!("Processing payment");
                }
            }
        }
        
        mod back_of_house {
            pub struct Breakfast {
                pub toast: String,
                seasonal_fruit: String, // Private field
            }
            
            impl Breakfast {
                pub fn summer(toast: &str) -> Breakfast {
                    Breakfast {
                        toast: String::from(toast),
                        seasonal_fruit: String::from("peaches"),
                    }
                }
            }
            
            pub enum Appetizer {
                Soup,
                Salad,
            }
            
            fn fix_incorrect_order() {
                cook_order();
                super::front_of_house::serving::serve_order(); // Using super
            }
            
            fn cook_order() {
                println!("Cooking order in kitchen");
            }
        }
        
        pub fn eat_at_restaurant() {
            // Relative path
            front_of_house::hosting::add_to_waitlist();
            
            // Order breakfast
            let mut meal = back_of_house::Breakfast::summer("Rye");
            meal.toast = String::from("Wheat");  // Can modify public field
            println!("I'd like {} toast please", meal.toast);
            
            // meal.seasonal_fruit = String::from("blueberries"); // Can't modify private field
            
            // Enum variants are public if enum is public
            let _order1 = back_of_house::Appetizer::Soup;
            let _order2 = back_of_house::Appetizer::Salad;
        }
    }
    
    restaurant::eat_at_restaurant();
    
    println!();
}

// Module definitions for use_statements function
mod sound {
    pub mod instrument {
        pub fn clarinet() {
            println!("Playing clarinet");
        }
        
        pub fn guitar() {
            println!("Playing guitar");
        }
    }
    
    pub mod voice {
        pub fn sing() {
            println!("Singing a song");
        }
    }
}

fn use_statements() {
    println!("3. Use Statements and Paths");
    
    // Bringing paths into scope with use
    use crate::sound::instrument;
    use crate::sound::voice::sing;
    use crate::sound::instrument::guitar as play_guitar; // Using alias
    
    instrument::clarinet();
    instrument::guitar();
    sing();
    play_guitar(); // Using alias
    
    // Enum example (use statements would bring variants into scope)
    #[derive(Debug)]
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }
    
    // Without use statements, need to specify full path
    let light1 = TrafficLight::Red;
    let light2 = TrafficLight::Yellow; 
    let light3 = TrafficLight::Green;
    
    // With use statements (commented to avoid compilation issues in this example):
    // use TrafficLight::{Red, Yellow}; // Bring specific variants into scope
    // use TrafficLight::*; // Bring all variants into scope (use sparingly)
    // Then you could use: let light1 = Red;
    
    println!("Traffic lights: {:?}, {:?}, {:?}", light1, light2, light3);
    
    println!();
}

// Module definitions for working_with_modules function
mod geometry {
    #[derive(Debug)]
    pub struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
        
        pub fn area(&self) -> u32 {
            self.width * self.height
        }
        
        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
    
    #[derive(Debug)]
    pub struct Circle {
        radius: f64,
    }
    
    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }
        
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    
    pub mod calculations {
        pub fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
            ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
        }
    }
}

fn working_with_modules() {
    println!("4. Working with Modules");
    
    use crate::geometry::{Rectangle, Circle};
    use crate::geometry::calculations::distance;
    
    // Using the geometry module
    let rect1 = Rectangle::new(30, 50);
    let rect2 = Rectangle::new(10, 40);
    let circle = Circle::new(5.0);
    
    println!("Rectangle 1: {:?}", rect1);
    println!("Rectangle 1 area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    println!("Circle: {:?}", circle);
    println!("Circle area: {:.2}", circle.area());
    
    let dist = distance(0.0, 0.0, 3.0, 4.0);
    println!("Distance between points: {:.2}", dist);
    
    println!();
}

fn external_crates_info() {
    println!("5. Working with External Crates");
    
    println!("To use external crates in a real Cargo project:");
    println!("1. Add dependency to Cargo.toml:");
    println!("   [dependencies]");
    println!("   rand = \"0.8.5\"");
    println!("   serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!();
    
    println!("2. Use the crate in your code:");
    println!("   use rand::Rng;");
    println!("   use serde::{{Serialize, Deserialize}};");
    println!();
    
    println!("3. Common external crates:");
    println!("   - rand: Random number generation");
    println!("   - serde: Serialization/deserialization");
    println!("   - tokio: Async runtime");
    println!("   - clap: Command line argument parsing");
    println!("   - reqwest: HTTP client");
    println!("   - chrono: Date and time handling");
    println!();
    
    // Demonstrating std library modules (which work like external crates)
    use std::collections::HashMap;
    use std::env;
    
    let mut map = HashMap::new();
    map.insert("example", "value");
    println!("HashMap example: {:?}", map);
    
    // Environment variables
    if let Ok(path) = env::var("PATH") {
        println!("PATH length: {}", path.len());
    }
    
    println!("Current directory: {:?}", env::current_dir());
    
    println!();
}

// Example of a more complex module structure
mod library {
    pub mod books {
        #[derive(Debug)]
        pub struct Book {
            pub title: String,
            pub author: String,
            pub isbn: String,
        }
        
        impl Book {
            pub fn new(title: &str, author: &str, isbn: &str) -> Self {
                Book {
                    title: title.to_string(),
                    author: author.to_string(),
                    isbn: isbn.to_string(),
                }
            }
            
            pub fn display_info(&self) {
                println!("\"{}\" by {} (ISBN: {})", self.title, self.author, self.isbn);
            }
        }
    }
    
    pub mod members {
        #[derive(Debug)]
        pub struct Member {
            pub name: String,
            pub id: u32,
        }
        
        impl Member {
            pub fn new(name: &str, id: u32) -> Self {
                Member {
                    name: name.to_string(),
                    id,
                }
            }
        }
    }
    
    pub mod transactions {
        use super::books::Book;
        use super::members::Member;
        
        pub fn checkout_book(book: &Book, member: &Member) {
            println!("Checking out book \"{}\" to {}", book.title, member.name);
        }
        
        pub fn return_book(book: &Book, member: &Member) {
            println!("{} returned \"{}\"", member.name, book.title);
        }
    }
}

// Example usage of complex module structure
fn _library_example() {
    use library::{books::Book, members::Member, transactions};
    
    let book = Book::new("The Rust Programming Language", "Steve Klabnik", "978-1593278281");
    let member = Member::new("Alice Johnson", 12345);
    
    book.display_info();
    transactions::checkout_book(&book, &member);
    transactions::return_book(&book, &member);
}

// Re-exporting for easier access
pub use library::books::Book;
pub use library::members::Member;

fn _re_export_example() {
    // Now we can use Book and Member directly without full path
    let _book = Book::new("Programming Rust", "Jim Blandy", "978-1491927281");
    let _member = Member::new("Bob Smith", 67890);
    
    println!("\n=== Modules and Crates Complete! ===");
}
