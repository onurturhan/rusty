// pattern_matching.rs
// Demonstrates Rust's powerful pattern matching

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    println!("=== Pattern Matching ===\n");

    // 1. Basic match with values
    println!("1. Basic Match:");
    let number = 7;
    match number {
        1 => println!("   One!"),
        2 | 3 => println!("   Two or Three!"),
        4..=6 => println!("   Four to Six!"),
        7 => println!("   Lucky seven!"),
        _ => println!("   Something else"),
    }
    println!();

    // 2. Match with enums
    println!("2. Match with Enums:");
    let coin = Coin::Quarter(UsState::California);
    let value = value_in_cents(coin);
    println!("   Coin value: {} cents\n", value);

    // 3. Match with Option
    println!("3. Match with Option:");
    let some_value = Some(5);
    let none_value: Option<i32> = None;
    
    match some_value {
        Some(i) => println!("   Got a value: {}", i),
        None => println!("   Got nothing"),
    }
    
    match none_value {
        Some(i) => println!("   Got a value: {}", i),
        None => println!("   Got nothing"),
    }
    println!();

    // 4. If let (concise pattern matching)
    println!("4. If Let:");
    let some_number = Some(3);
    
    if let Some(3) = some_number {
        println!("   Three!");
    }
    
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("   Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("   Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("   Using purple as the background color");
        } else {
            println!("   Using orange as the background color");
        }
    } else {
        println!("   Using blue as the background color");
    }
    println!();

    // 5. While let
    println!("5. While Let:");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("   Popped: {}", top);
    }
    println!();

    // 6. Destructuring structs
    println!("6. Destructuring Structs:");
    let point = Point { x: 0, y: 7 };
    let Point { x, y } = point;
    println!("   x: {}, y: {}", x, y);
    
    match point {
        Point { x, y: 0 } => println!("   On the x-axis at {}", x),
        Point { x: 0, y } => println!("   On the y-axis at {}", y),
        Point { x, y } => println!("   On neither axis: ({}, {})", x, y),
    }
    println!();

    // 7. Destructuring enums
    println!("7. Destructuring Enums:");
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 0, 0),
    ];
    
    for msg in messages {
        process_message(msg);
    }
    println!();

    // 8. Complex patterns with tuples
    println!("8. Complex Patterns:");
    let numbers = (2, 4, 8, 16, 32);
    
    match numbers {
        (first, .., last) => {
            println!("   Some numbers: {} and {}", first, last);
        }
    }
    println!();

    // 9. Match guards
    println!("9. Match Guards:");
    let num = Some(4);
    
    match num {
        Some(x) if x < 5 => println!("   Less than five: {}", x),
        Some(x) => println!("   Greater than or equal to five: {}", x),
        None => (),
    }
    
    let x = 4;
    let y = true;
    
    match x {
        4 | 5 | 6 if y => println!("   Yes, x is 4, 5, or 6 and y is true"),
        _ => println!("   No match"),
    }
    println!();

    println!("=== End of Pattern Matching ===");
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("   Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("   State quarter from {:?}!", state);
            25
        }
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("   The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("   Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("   Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("   Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}
