// Exercise 1: Variables and Data Types
// Complete the TODOs below

fn main() {
    // TODO: Declare an immutable variable 'name' with your name
    // let name = ...;
    
    // TODO: Declare a mutable variable 'age' and set it to your age
    // let mut age = ...;
    
    // TODO: Declare a constant MAX_SCORE with value 100
    // const MAX_SCORE: ... = ...;
    
    // TODO: Create a variable 'score' and then shadow it with a different value
    // let score = ...;
    // let score = ...;
    
    // TODO: Print all variables using println! macro
    // println!("Name: {}", name);
    // println!("Age: {}", age);
    // println!("Max Score: {}", MAX_SCORE);
    // println!("Score: {}", score);
    
    // TODO: Try to modify the age variable and print it again
    // age = ...;
    // println!("New age: {}", age);
}

/* 
SOLUTION (uncomment when ready to check):

fn main() {
    let name = "Your Name";
    let mut age = 25;
    const MAX_SCORE: u32 = 100;
    
    let score = 80;
    let score = score + 10; // shadowing
    
    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Max Score: {}", MAX_SCORE);
    println!("Score: {}", score);
    
    age = 26;
    println!("New age: {}", age);
}
*/
