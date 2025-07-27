// control_flow.rs
// Rust control flow examples: if/else, loops, and match

fn main() {
    println!("=== Control Flow Examples ===\n");

    // If/else statements
    println!("1. If/Else Statements:");
    let number = 42;
    
    if number < 50 {
        println!("   {} is less than 50", number);
    } else if number == 50 {
        println!("   {} equals 50", number);
    } else {
        println!("   {} is greater than 50", number);
    }

    // If as expression (returns value)
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("   If expression result: {}\n", result);

    // For loops
    println!("2. For Loops:");
    print!("   Range 1..5: ");
    for i in 1..5 {
        print!("{} ", i);
    }
    println!();

    print!("   Array iteration: ");
    let arr = [10, 20, 30, 40, 50];
    for element in arr {
        print!("{} ", element);
    }
    println!();

    print!("   With index: ");
    for (index, value) in arr.iter().enumerate() {
        print!("{}:{} ", index, value);
    }
    println!("\n");

    // While loops
    println!("3. While Loops:");
    let mut counter = 3;
    print!("   Countdown: ");
    while counter > 0 {
        print!("{} ", counter);
        counter -= 1;
    }
    println!("Go!\n");

    // Loop (infinite loop with break)
    println!("4. Loop with Break:");
    let mut x = 0;
    let result = loop {
        x += 1;
        if x == 10 {
            break x * 2; // Return value from loop
        }
    };
    println!("   Loop result: {}\n", result);

    // Match expressions
    println!("5. Match Expressions:");
    let dice_roll = 4;
    match dice_roll {
        1 => println!("   Rolled a 1 - Try again!"),
        2 | 3 => println!("   Rolled {} - Not bad", dice_roll),
        4..=6 => println!("   Rolled {} - Good roll!", dice_roll),
        _ => println!("   Invalid dice roll"),
    }

    // Match with enum
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West,
    }

    let dir = Direction::North;
    match dir {
        Direction::North => println!("   Going North ⬆️"),
        Direction::South => println!("   Going South ⬇️"),
        Direction::East => println!("   Going East ➡️"),
        Direction::West => println!("   Going West ⬅️"),
    }

    // Match with Option
    let maybe_number: Option<i32> = Some(5);
    match maybe_number {
        Some(n) => println!("   Found number: {}", n),
        None => println!("   No number found"),
    }
    println!();

    // Nested control flow example
    println!("6. Nested Control Flow:");
    for i in 1..=5 {
        let status = if i % 2 == 0 {
            "even"
        } else {
            "odd"
        };
        
        let description = match i {
            1 => "first",
            2 => "second", 
            3 => "third",
            4 => "fourth",
            5 => "fifth",
            _ => "unknown",
        };
        
        println!("   {} is {} and {}", i, status, description);
    }
}
