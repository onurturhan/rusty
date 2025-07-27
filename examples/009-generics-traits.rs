// 009-generics-traits.rs
// This example demonstrates Rust's generic types and trait system
// Run with: rustc examples/009-generics-traits.rs -o 009-generics-traits && ./009-generics-traits

use std::fmt::Display;
use std::cmp::PartialOrd;

fn main() {
    println!("=== Rust Generics and Traits Examples ===\n");

    // Generic functions
    generic_functions();
    
    // Generic structs
    generic_structs();
    
    // Traits - defining shared behavior
    trait_examples();
    
    // Trait bounds and where clauses
    trait_bounds();
    
    // Advanced trait features
    advanced_traits();
}

fn generic_functions() {
    println!("1. Generic Functions");
    
    // Simple generic function
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    
    let numbers = vec![34, 50, 25, 100, 65];
    let result = largest(&numbers);
    println!("Largest number: {}", result);
    
    let chars = vec!['y', 'm', 'a', 'q'];
    let result = largest(&chars);
    println!("Largest char: {}", result);
    
    // Generic function with multiple type parameters
    fn compare_and_display<T, U>(t: &T, u: &U)
    where
        T: Display + PartialEq,
        U: Display + PartialEq,
    {
        println!("Comparing: {} and {}", t, u);
    }
    
    compare_and_display(&42, &"hello");
    
    println!();
}

fn generic_structs() {
    println!("2. Generic Structs and Enums");
    
    // Generic struct
    #[derive(Debug)]
    struct Point<T> {
        x: T,
        y: T,
    }
    
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self {
            Point { x, y }
        }
        
        fn x(&self) -> &T {
            &self.x
        }
    }
    
    // Specific implementation for f32
    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }
    
    let integer_point = Point::new(5, 10);
    let float_point = Point::new(1.0, 4.0);
    
    println!("Integer point: {:?}", integer_point);
    println!("Float point: {:?}", float_point);
    println!("Distance from origin: {:.2}", float_point.distance_from_origin());
    
    // Generic struct with multiple type parameters
    #[derive(Debug)]
    struct Pair<T, U> {
        first: T,
        second: U,
    }
    
    impl<T, U> Pair<T, U> {
        fn new(first: T, second: U) -> Self {
            Pair { first, second }
        }
        
        fn into_tuple(self) -> (T, U) {
            (self.first, self.second)
        }
    }
    
    let mixed_pair = Pair::new("hello", 42);
    println!("Mixed pair: {:?}", mixed_pair);
    
    // Generic enum
    #[derive(Debug)]
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    
    let success: Result<i32, &str> = Result::Ok(200);
    let failure: Result<i32, &str> = Result::Err("Something went wrong");
    
    println!("Success: {:?}", success);
    println!("Failure: {:?}", failure);
    
    println!();
}

fn trait_examples() {
    println!("3. Traits - Defining Shared Behavior");
    
    // Define a trait
    trait Summary {
        fn summarize(&self) -> String;
        
        // Default implementation
        fn summarize_author(&self) -> String {
            String::from("(Read more...)")
        }
    }
    
    // Structs that implement the trait
    struct NewsArticle {
        headline: String,
        location: String,
        author: String,
        content: String,
    }
    
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
        
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }
    }
    
    struct Tweet {
        username: String,
        content: String,
        reply: bool,
        retweet: bool,
    }
    
    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }
    
    let article = NewsArticle {
        headline: String::from("Rust 1.70 Released!"),
        location: String::from("Internet"),
        author: String::from("Rust Team"),
        content: String::from("The Rust team is happy to announce..."),
    };
    
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust is memory safe by default!"),
        reply: false,
        retweet: false,
    };
    
    println!("Article summary: {}", article.summarize());
    println!("Article author: {}", article.summarize_author());
    println!("Tweet summary: {}", tweet.summarize());
    println!("Tweet author: {}", tweet.summarize_author()); // Uses default
    
    // Function that takes trait objects
    fn notify(item: &dyn Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    
    notify(&article);
    notify(&tweet);
    
    println!();
}

fn trait_bounds() {
    println!("4. Trait Bounds and Where Clauses");
    
    // Function with trait bounds
    fn notify_and_display<T: Summary + Display>(item: &T) {
        println!("Notification: {}", item.summarize());
        println!("Display: {}", item);
    }
    
    // Using where clauses for cleaner syntax
    fn some_function<T, U>(_t: &T, _u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + std::fmt::Debug,
    {
        42
    }
    
    // Conditional trait implementation
    struct Wrapper<T>(T);
    
    impl<T: Display> Display for Wrapper<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Wrapper({})", self.0)
        }
    }
    
    let wrapped_string = Wrapper(String::from("hello"));
    let wrapped_number = Wrapper(42);
    
    println!("Wrapped string: {}", wrapped_string);
    println!("Wrapped number: {}", wrapped_number);
    
    // Return types with trait bounds
    fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
        if switch {
            Box::new(NewsArticle {
                headline: String::from("Breaking News"),
                location: String::from("Local"),
                author: String::from("Reporter"),
                content: String::from("Something happened"),
            })
        } else {
            Box::new(Tweet {
                username: String::from("user"),
                content: String::from("Quick update"),
                reply: false,
                retweet: false,
            })
        }
    }
    
    let item = returns_summarizable(true);
    println!("Dynamic summary: {}", item.summarize());
    
    println!();
}

fn advanced_traits() {
    println!("5. Advanced Trait Features");
    
    // Associated types
    trait Iterator {
        type Item;
        
        fn next(&mut self) -> Option<Self::Item>;
    }
    
    struct Counter {
        count: usize,
        max: usize,
    }
    
    impl Counter {
        fn new(max: usize) -> Counter {
            Counter { count: 0, max }
        }
    }
    
    impl Iterator for Counter {
        type Item = usize;
        
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                let result = self.count;
                self.count += 1;
                Some(result)
            } else {
                None
            }
        }
    }
    
    let mut counter = Counter::new(3);
    while let Some(value) = counter.next() {
        println!("Counter value: {}", value);
    }
    
    // Operator overloading with traits
    use std::ops::Add;
    
    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    
    impl Add for Point {
        type Output = Point;
        
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    
    println!("Point addition: {:?} + {:?} = {:?}", p1, p2, p3);
    
    // Supertraits
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    
    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    
    impl OutlinePrint for Point {}
    
    let point = Point { x: 5, y: 7 };
    point.outline_print();
    
    // Newtype pattern for external trait implementation
    struct Millimeters(i32);
    
    impl Display for Millimeters {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}mm", self.0)
        }
    }
    
    let distance = Millimeters(42);
    println!("Distance: {}", distance);
    
    println!("\n=== Generics and Traits Complete! ===");
}

// Additional trait examples
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Draw for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

// This could be used to store different drawable objects
fn draw_shapes(shapes: Vec<Box<dyn Draw>>) {
    for shape in shapes {
        shape.draw();
    }
}
