// structs_enums.rs
// Demonstrates Rust structs and enums

// Define structs
#[derive(Debug)] // Allows printing with {:?}
struct Person {
    name: String,
    age: u32,
    email: String,
}

#[derive(Debug)]
struct Point(i32, i32); // Tuple struct

#[derive(Debug)]
struct Unit; // Unit struct

// Define enums
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("=== Structs and Enums ===\n");

    // 1. Creating and using structs
    println!("1. Structs:");
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
        email: String::from("alice@example.com"),
    };
    
    println!("   Person: {:?}", person1);
    println!("   Name: {}, Age: {}", person1.name, person1.age);

    // Struct update syntax
    let person2 = Person {
        name: String::from("Bob"),
        email: String::from("bob@example.com"),
        ..person1 // Take remaining fields from person1
    };
    println!("   Person2: {:?}\n", person2);

    // 2. Tuple structs
    println!("2. Tuple Structs:");
    let origin = Point(0, 0);
    let point = Point(3, 4);
    println!("   Origin: {:?}", origin);
    println!("   Point: {:?}", point);
    println!("   Point coordinates: ({}, {})\n", point.0, point.1);

    // 3. Unit struct
    println!("3. Unit Struct:");
    let unit = Unit;
    println!("   Unit struct: {:?}\n", unit);

    // 4. Enums
    println!("4. Simple Enums:");
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("   IPv4: {:?}", four);
    println!("   IPv6: {:?}\n", six);

    // 5. Enums with data
    println!("5. Enums with Data:");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("   Home: {:?}", home);
    println!("   Loopback: {:?}\n", loopback);

    // 6. Complex enums
    println!("6. Complex Enums:");
    let quit = Message::Quit;
    let move_msg = Message::Move { x: 10, y: 20 };
    let write_msg = Message::Write(String::from("Hello"));
    let color_msg = Message::ChangeColor(255, 0, 0);

    println!("   Messages:");
    println!("     {:?}", quit);
    println!("     {:?}", move_msg);
    println!("     {:?}", write_msg);
    println!("     {:?}\n", color_msg);

    // 7. Methods on structs
    println!("7. Struct Methods:");
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("   Rectangle: {:?}", rect);
    println!("   Area: {}", rect.area());
    println!("   Is square: {}\n", rect.is_square());

    // 8. Associated functions
    println!("8. Associated Functions:");
    let square = Rectangle::square(25);
    println!("   Square: {:?}", square);
    println!("   Square area: {}\n", square.area());

    // 9. Option enum (built-in)
    println!("9. Option Enum:");
    let some_number = Some(5);
    let some_string = Some("hello");
    let absent_number: Option<i32> = None;

    println!("   Some number: {:?}", some_number);
    println!("   Some string: {:?}", some_string);
    println!("   Absent number: {:?}\n", absent_number);

    // 10. Pattern matching with enums
    println!("10. Pattern Matching:");
    process_message(quit);
    process_message(move_msg);
    process_message(write_msg);
    process_message(color_msg);

    println!("\n=== End of Structs and Enums ===");
}

// Struct with methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // Associated function (doesn't take self)
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Function that processes messages
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("   Processing: Quit message received"),
        Message::Move { x, y } => println!("   Processing: Move to ({}, {})", x, y),
        Message::Write(text) => println!("   Processing: Write '{}'", text),
        Message::ChangeColor(r, g, b) => println!("   Processing: Change color to RGB({}, {}, {})", r, g, b),
    }
}
