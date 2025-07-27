// 010-lifetimes.rs
// This example demonstrates Rust's lifetime system for managing references
// Run with: rustc examples/010-lifetimes.rs -o 010-lifetimes && ./010-lifetimes

fn main() {
    println!("=== Rust Lifetimes Examples ===\n");

    // Basic lifetime concepts
    basic_lifetimes();
    
    // Functions with lifetime parameters
    lifetime_functions();
    
    // Structs with lifetimes
    lifetime_structs();
    
    // Lifetime elision rules
    lifetime_elision();
    
    // Static lifetimes
    static_lifetimes();
}

fn basic_lifetimes() {
    println!("1. Basic Lifetime Concepts");
    
    // Every reference has a lifetime
    let x = 5;                // x comes into scope
    let r = &x;               // r is a reference to x
    println!("r: {}", r);     // r is valid here
    // Both x and r go out of scope here
    
    // The borrow checker prevents dangling references
    let _reference_to_nothing: Option<&i32> = None;
    {
        let x = 5;
        // _reference_to_nothing = Some(&x); // This would fail - x doesn't live long enough
    }
    // println!("reference: {:?}", _reference_to_nothing); // Would work but x is dropped
    
    // Lifetime annotations in action
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);
    
    println!();
}

// Function with lifetime parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetime_functions() {
    println!("2. Functions with Lifetime Parameters");
    
    // Function that takes references and returns a reference
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        
        &s[..]
    }
    
    let sentence = "Hello world from Rust";
    let word = first_word(sentence);
    println!("First word: {}", word);
    
    // Multiple lifetime parameters
    fn compare_lengths<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            // Can't return y here because it has a different lifetime
            x // Return x instead to satisfy the lifetime constraint
        }
    }
    
    let short = "hi";
    let long = "this is a longer string";
    let result = compare_lengths(long, short);
    println!("Result from compare_lengths: {}", result);
    
    // Function with lifetime and generic parameters
    fn get_first_element<'a, T>(slice: &'a [T]) -> Option<&'a T> {
        slice.first()
    }
    
    let numbers = vec![1, 2, 3, 4, 5];
    if let Some(first) = get_first_element(&numbers) {
        println!("First number: {}", first);
    }
    
    println!();
}

fn lifetime_structs() {
    println!("3. Structs with Lifetimes");
    
    // Struct that holds a reference
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    
    impl<'a> ImportantExcerpt<'a> {
        // Method with lifetime elision
        fn level(&self) -> i32 {
            3
        }
        
        // Method that returns reference with same lifetime as self
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    
    println!("Important excerpt: {:?}", i);
    println!("Level: {}", i.level());
    
    let part = i.announce_and_return_part("We have some important news!");
    println!("Returned part: {}", part);
    
    // Multiple references in struct
    #[derive(Debug)]
    struct TwoStrings<'a, 'b> {
        first: &'a str,
        second: &'b str,
    }
    
    let str1 = "First string";
    let str2 = "Second string";
    let two = TwoStrings {
        first: str1,
        second: str2,
    };
    
    println!("Two strings: {:?}", two);
    
    println!();
}

fn lifetime_elision() {
    println!("4. Lifetime Elision Rules");
    
    // Rule 1: Each parameter that is a reference gets its own lifetime parameter
    // fn first_word(s: &str) -> &str {  // Becomes: fn first_word<'a>(s: &'a str) -> &str
    
    // Rule 2: If there's exactly one input lifetime parameter, 
    // it's assigned to all output lifetime parameters
    fn get_prefix(text: &str) -> &str {  // Becomes: fn get_prefix<'a>(text: &'a str) -> &'a str
        &text[..3]
    }
    
    let text = "Hello, World!";
    let prefix = get_prefix(text);
    println!("Prefix: {}", prefix);
    
    // Rule 3: If one of the parameters is &self or &mut self,
    // the lifetime of self is assigned to all output lifetime parameters
    
    struct StringHolder {
        content: String,
    }
    
    impl StringHolder {
        // No explicit lifetime needed due to elision rules
        fn get_content(&self) -> &str {
            &self.content
        }
        
        fn get_first_char(&self) -> Option<char> {
            self.content.chars().next()
        }
    }
    
    let holder = StringHolder {
        content: String::from("Rust programming"),
    };
    
    println!("Content: {}", holder.get_content());
    if let Some(first) = holder.get_first_char() {
        println!("First character: {}", first);
    }
    
    println!();
}

fn static_lifetimes() {
    println!("5. Static Lifetimes");
    
    // 'static lifetime means the reference can live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
    println!("Static string: {}", s);
    
    // String literals have 'static lifetime
    fn get_static_str() -> &'static str {
        "This is a string literal with static lifetime"
    }
    
    println!("Static function result: {}", get_static_str());
    
    // Static variables
    static HELLO_WORLD: &str = "Hello, world!";
    println!("Static variable: {}", HELLO_WORLD);
    
    // Generic type parameters, trait bounds, and lifetimes together
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: std::fmt::Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string is long");
    let string2 = "xyz";
    let announcement = "Today is someone's birthday!";
    
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        announcement,
    );
    println!("The longest string is: {}", result);
    
    println!("\n=== Lifetimes Complete! ===");
}

// Examples of common lifetime patterns

// Pattern 1: Returning one of the input references
fn choose_first<'a>(first: &'a str, _second: &str) -> &'a str {
    first
}

// Pattern 2: Struct holding multiple references
struct Context<'s> {
    text: &'s str,
}

impl<'s> Context<'s> {
    fn new(text: &'s str) -> Self {
        Context { text }
    }
    
    fn get_text(&self) -> &str {
        self.text
    }
}

// Pattern 3: Method that borrows self and returns reference
struct Parser<'s> {
    input: &'s str,
    pos: usize,
}

impl<'s> Parser<'s> {
    fn new(input: &'s str) -> Self {
        Parser { input, pos: 0 }
    }
    
    fn peek(&self) -> Option<&str> {
        if self.pos < self.input.len() {
            Some(&self.input[self.pos..self.pos + 1])
        } else {
            None
        }
    }
}
