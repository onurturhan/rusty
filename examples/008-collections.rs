// 008-collections.rs
// This example demonstrates Rust's core collection types: Vec, HashMap, HashSet, etc.
// Run with: rustc examples/008-collections.rs -o 008-collections && ./008-collections

use std::collections::{HashMap, HashSet, VecDeque, BTreeMap};

fn main() {
    println!("=== Rust Collections Examples ===\n");

    // Vec<T> - Dynamic arrays
    vector_examples();
    
    // HashMap<K, V> - Hash tables
    hashmap_examples();
    
    // HashSet<T> - Unique values
    hashset_examples();
    
    // Other useful collections
    other_collections();
    
    // Advanced collection operations
    advanced_operations();
}

fn vector_examples() {
    println!("1. Vec<T> (Vectors - Dynamic Arrays)");
    
    // Creating vectors
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    
    // Macro syntax
    let fruits = vec!["apple", "banana", "cherry"];
    
    // With type annotation
    let mut scores: Vec<i32> = Vec::new();
    scores.extend(vec![85, 92, 78, 95]);
    
    println!("Numbers: {:?}", numbers);
    println!("Fruits: {:?}", fruits);
    println!("Scores: {:?}", scores);
    
    // Accessing elements
    println!("First fruit: {}", fruits[0]); // Panics if out of bounds
    println!("Second score: {:?}", scores.get(1)); // Returns Option<&T>
    
    // Iteration
    print!("Squares: ");
    for num in &numbers {
        print!("{} ", num * num);
    }
    println!();
    
    // Mutable iteration
    for score in &mut scores {
        *score += 5; // Add 5 bonus points
    }
    println!("Bonus scores: {:?}", scores);
    
    // Vector methods
    println!("Length: {}", scores.len());
    println!("Is empty: {}", scores.is_empty());
    println!("Capacity: {}", scores.capacity());
    
    // Remove elements
    scores.pop(); // Remove last element
    println!("After pop: {:?}", scores);
    
    println!();
}

fn hashmap_examples() {
    println!("2. HashMap<K, V> (Hash Tables)");
    
    // Creating HashMaps
    let mut student_grades = HashMap::new();
    student_grades.insert("Alice", 95);
    student_grades.insert("Bob", 87);
    student_grades.insert("Charlie", 92);
    
    // From iterator
    let teams = vec![("Red", 3), ("Blue", 10), ("Yellow", 50)];
    let team_scores: HashMap<&str, i32> = teams.into_iter().collect();
    
    println!("Student grades: {:?}", student_grades);
    println!("Team scores: {:?}", team_scores);
    
    // Accessing values
    match student_grades.get("Alice") {
        Some(grade) => println!("Alice's grade: {}", grade),
        None => println!("Alice not found"),
    }
    
    // Insert or update
    student_grades.entry("Diana").or_insert(88);
    student_grades.entry("Alice").or_insert(0); // Won't overwrite existing
    
    // Update based on old value
    let alice_grade = student_grades.entry("Alice").or_insert(0);
    *alice_grade += 5; // Add bonus points
    
    println!("Updated grades: {:?}", student_grades);
    
    // Iteration
    println!("All students:");
    for (name, grade) in &student_grades {
        println!("  {} -> {}", name, grade);
    }
    
    // Word counting example
    let text = "hello world hello rust world";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word count: {:?}", word_count);
    println!();
}

fn hashset_examples() {
    println!("3. HashSet<T> (Unique Values)");
    
    let mut unique_numbers = HashSet::new();
    unique_numbers.insert(1);
    unique_numbers.insert(2);
    unique_numbers.insert(3);
    unique_numbers.insert(2); // Duplicate - won't be added
    
    println!("Unique numbers: {:?}", unique_numbers);
    println!("Contains 2: {}", unique_numbers.contains(&2));
    println!("Length: {}", unique_numbers.len());
    
    // From vector (removes duplicates)
    let numbers = vec![1, 2, 2, 3, 3, 3, 4];
    let unique: HashSet<i32> = numbers.into_iter().collect();
    println!("Unique from vector: {:?}", unique);
    
    // Set operations
    let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
    let set2: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
    
    let intersection: HashSet<_> = set1.intersection(&set2).collect();
    let union: HashSet<_> = set1.union(&set2).collect();
    let difference: HashSet<_> = set1.difference(&set2).collect();
    
    println!("Set 1: {:?}", set1);
    println!("Set 2: {:?}", set2);
    println!("Intersection: {:?}", intersection);
    println!("Union: {:?}", union);
    println!("Difference (set1 - set2): {:?}", difference);
    
    println!();
}

fn other_collections() {
    println!("4. Other Useful Collections");
    
    // VecDeque - Double-ended queue
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_front(0);
    println!("VecDeque: {:?}", deque);
    
    if let Some(front) = deque.pop_front() {
        println!("Popped from front: {}", front);
    }
    println!("After pop_front: {:?}", deque);
    
    // BTreeMap - Sorted map
    let mut sorted_map = BTreeMap::new();
    sorted_map.insert("Charlie", 92);
    sorted_map.insert("Alice", 95);
    sorted_map.insert("Bob", 87);
    
    println!("BTreeMap (sorted by key):");
    for (name, grade) in &sorted_map {
        println!("  {} -> {}", name, grade);
    }
    
    // String collection
    let mut languages = Vec::new();
    languages.push(String::from("Rust"));
    languages.push(String::from("Go"));
    languages.push(String::from("Python"));
    
    println!("Programming languages: {:?}", languages);
    
    println!();
}

fn advanced_operations() {
    println!("5. Advanced Collection Operations");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Functional programming style operations
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .cloned()
        .collect();
    
    let squares: Vec<i32> = numbers
        .iter()
        .map(|x| x * x)
        .collect();
    
    let sum: i32 = numbers.iter().sum();
    let product: i32 = numbers.iter().product();
    
    println!("Original: {:?}", numbers);
    println!("Evens: {:?}", evens);
    println!("Squares: {:?}", squares);
    println!("Sum: {}", sum);
    println!("Product: {}", product);
    
    // Find operations
    let first_big = numbers.iter().find(|&&x| x > 5);
    let all_positive = numbers.iter().all(|&x| x > 0);
    let any_negative = numbers.iter().any(|&x| x < 0);
    
    println!("First number > 5: {:?}", first_big);
    println!("All positive: {}", all_positive);
    println!("Any negative: {}", any_negative);
    
    // Grouping with HashMap
    let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
    let mut by_first_letter: HashMap<char, Vec<&str>> = HashMap::new();
    
    for word in words {
        let first_char = word.chars().next().unwrap();
        by_first_letter.entry(first_char).or_insert(Vec::new()).push(word);
    }
    
    println!("Words grouped by first letter:");
    for (letter, word_list) in &by_first_letter {
        println!("  {}: {:?}", letter, word_list);
    }
    
    // Chaining operations
    let processed: Vec<String> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)    // Keep evens
        .map(|x| x * x)             // Square them
        .filter(|&x| x > 10)        // Keep if > 10
        .map(|x| format!("{}²", (x as f64).sqrt() as i32)) // Format as squares
        .collect();
    
    println!("Processed (even numbers squared > 10): {:?}", processed);
    
    println!("\n=== Collections Complete! ===");
}
