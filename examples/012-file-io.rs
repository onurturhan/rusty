// 012-file-io.rs
// This example demonstrates file I/O operations in Rust
// Run with: rustc examples/012-file-io.rs -o 012-file-io && ./012-file-io

use std::fs;
use std::fs::File;
use std::io::{self, Read, Write, BufRead, BufReader, BufWriter};
use std::path::Path;

fn main() {
    println!("=== Rust File I/O Examples ===\n");

    // Basic file operations
    basic_file_operations();
    
    // Reading files
    reading_files();
    
    // Writing files
    writing_files();
    
    // Buffered I/O
    buffered_io();
    
    // Error handling with files
    file_error_handling();
    
    // Working with paths
    path_operations();
    
    // Directory operations
    directory_operations();
}

fn basic_file_operations() {
    println!("1. Basic File Operations");
    
    // Create a simple text file
    let filename = "example.txt";
    let content = "Hello, Rust!\nThis is a test file.\nFile I/O is powerful!";
    
    // Write to file using fs::write (simplest way)
    match fs::write(filename, content) {
        Ok(()) => println!("✓ Successfully wrote to {}", filename),
        Err(e) => println!("✗ Error writing file: {}", e),
    }
    
    // Read from file using fs::read_to_string (simplest way)
    match fs::read_to_string(filename) {
        Ok(contents) => {
            println!("✓ File contents:");
            println!("{}", contents);
        }
        Err(e) => println!("✗ Error reading file: {}", e),
    }
    
    // Clean up
    let _ = fs::remove_file(filename);
    
    println!();
}

fn reading_files() {
    println!("2. Reading Files (Various Methods)");
    
    // Create test file
    let filename = "read_test.txt";
    let test_content = "Line 1: Hello World\nLine 2: Rust is awesome\nLine 3: File I/O examples\nLine 4: The end";
    fs::write(filename, test_content).expect("Failed to create test file");
    
    // Method 1: Read entire file as string
    println!("Method 1: fs::read_to_string()");
    match fs::read_to_string(filename) {
        Ok(contents) => println!("Full content:\n{}", contents),
        Err(e) => println!("Error: {}", e),
    }
    
    // Method 2: Read as bytes
    println!("\nMethod 2: fs::read() - as bytes");
    match fs::read(filename) {
        Ok(bytes) => {
            println!("File size: {} bytes", bytes.len());
            // Convert first 20 bytes to string for display
            let preview = String::from_utf8_lossy(&bytes[..std::cmp::min(20, bytes.len())]);
            println!("Preview: {}", preview);
        }
        Err(e) => println!("Error: {}", e),
    }
    
    // Method 3: Using File and Read trait
    println!("\nMethod 3: File::open() and Read trait");
    match File::open(filename) {
        Ok(mut file) => {
            let mut contents = String::new();
            match file.read_to_string(&mut contents) {
                Ok(bytes_read) => {
                    println!("Read {} bytes", bytes_read);
                    println!("Content: {}", contents);
                }
                Err(e) => println!("Error reading: {}", e),
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
    
    // Method 4: Reading line by line
    println!("\nMethod 4: Reading line by line");
    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for (line_num, line) in reader.lines().enumerate() {
                match line {
                    Ok(content) => println!("Line {}: {}", line_num + 1, content),
                    Err(e) => println!("Error reading line: {}", e),
                }
            }
        }
        Err(e) => println!("Error opening file: {}", e),
    }
    
    // Clean up
    let _ = fs::remove_file(filename);
    
    println!();
}

fn writing_files() {
    println!("3. Writing Files (Various Methods)");
    
    let filename = "write_test.txt";
    
    // Method 1: fs::write (overwrites existing file)
    println!("Method 1: fs::write()");
    let content = "This is written using fs::write\n";
    match fs::write(filename, content) {
        Ok(()) => println!("✓ Successfully wrote using fs::write"),
        Err(e) => println!("✗ Error: {}", e),
    }
    
    // Method 2: File::create and Write trait
    println!("\nMethod 2: File::create() and Write trait");
    match File::create(filename) {
        Ok(mut file) => {
            let additional_content = "This replaces the previous content\nNew line added\n";
            match file.write_all(additional_content.as_bytes()) {
                Ok(()) => println!("✓ Successfully wrote using File::create"),
                Err(e) => println!("✗ Error writing: {}", e),
            }
        }
        Err(e) => println!("✗ Error creating file: {}", e),
    }
    
    // Method 3: Append mode
    println!("\nMethod 3: Append mode");
    use std::fs::OpenOptions;
    
    match OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)
    {
        Ok(mut file) => {
            let append_content = "This line is appended\nAnother appended line\n";
            match file.write_all(append_content.as_bytes()) {
                Ok(()) => println!("✓ Successfully appended to file"),
                Err(e) => println!("✗ Error appending: {}", e),
            }
        }
        Err(e) => println!("✗ Error opening file for append: {}", e),
    }
    
    // Show final content
    if let Ok(final_content) = fs::read_to_string(filename) {
        println!("\nFinal file content:");
        println!("{}", final_content);
    }
    
    // Clean up
    let _ = fs::remove_file(filename);
    
    println!();
}

fn buffered_io() {
    println!("4. Buffered I/O for Better Performance");
    
    let filename = "buffered_test.txt";
    
    // Writing with BufWriter
    println!("Writing with BufWriter:");
    match File::create(filename) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            
            // Write multiple lines
            for i in 1..=5 {
                let line = format!("Buffered line {}: Hello from BufWriter!\n", i);
                match writer.write_all(line.as_bytes()) {
                    Ok(()) => (),
                    Err(e) => println!("Error writing line {}: {}", i, e),
                }
            }
            
            // Important: flush to ensure all data is written
            match writer.flush() {
                Ok(()) => println!("✓ Successfully wrote and flushed buffered content"),
                Err(e) => println!("✗ Error flushing: {}", e),
            }
        }
        Err(e) => println!("✗ Error creating file: {}", e),
    }
    
    // Reading with BufReader
    println!("\nReading with BufReader:");
    match File::open(filename) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut line_count = 0;
            
            // Read line by line efficiently
            loop {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => break, // EOF reached
                    Ok(_) => {
                        line_count += 1;
                        print!("Read: {}", line); // line already contains \n
                    }
                    Err(e) => {
                        println!("Error reading line: {}", e);
                        break;
                    }
                }
            }
            
            println!("Total lines read: {}", line_count);
        }
        Err(e) => println!("✗ Error opening file: {}", e),
    }
    
    // Clean up
    let _ = fs::remove_file(filename);
    
    println!();
}

fn file_error_handling() {
    println!("5. File Error Handling");
    
    // Attempting to read non-existent file
    let nonexistent = "this_file_does_not_exist.txt";
    
    println!("Attempting to read non-existent file:");
    match fs::read_to_string(nonexistent) {
        Ok(content) => println!("Content: {}", content),
        Err(e) => {
            println!("✗ Expected error: {}", e);
            println!("Error kind: {:?}", e.kind());
        }
    }
    
    // Different ways to handle errors
    println!("\nDifferent error handling approaches:");
    
    // Using Result and match
    match File::open(nonexistent) {
        Ok(_) => println!("File opened successfully"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => println!("File not found - using match"),
            io::ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("Other error: {}", e),
        },
    }
    
    // Using unwrap_or_else
    let _file = File::open(nonexistent).unwrap_or_else(|e| {
        println!("Using unwrap_or_else - creating new file due to: {}", e);
        File::create("fallback.txt").expect("Failed to create fallback file")
    });
    
    // Using map_err for error transformation
    let result: Result<String, String> = fs::read_to_string(nonexistent)
        .map_err(|e| format!("Custom error: Failed to read file - {}", e));
    
    match result {
        Ok(content) => println!("Content: {}", content),
        Err(custom_error) => println!("✗ {}", custom_error),
    }
    
    // Clean up
    let _ = fs::remove_file("fallback.txt");
    
    println!();
}

fn path_operations() {
    println!("6. Working with Paths");
    
    use std::path::PathBuf;
    
    // Creating paths
    let mut path = PathBuf::new();
    path.push("home");
    path.push("user");
    path.push("documents");
    path.push("file.txt");
    
    println!("Constructed path: {:?}", path);
    println!("Path as string: {:?}", path.to_str());
    
    // Path components
    if let Some(filename) = path.file_name() {
        println!("Filename: {:?}", filename);
    }
    
    if let Some(parent) = path.parent() {
        println!("Parent directory: {:?}", parent);
    }
    
    if let Some(extension) = path.extension() {
        println!("Extension: {:?}", extension);
    }
    
    // Path manipulation
    let mut new_path = path.clone();
    new_path.set_extension("rs");
    println!("Changed extension: {:?}", new_path);
    
    // Working with current directory
    match std::env::current_dir() {
        Ok(current_path) => {
            println!("Current directory: {:?}", current_path);
            
            let relative_path = current_path.join("test_file.txt");
            println!("Joined path: {:?}", relative_path);
        }
        Err(e) => println!("Error getting current directory: {}", e),
    }
    
    // Check if path exists
    let test_path = Path::new("Cargo.toml");
    if test_path.exists() {
        println!("Cargo.toml exists in current directory");
        
        if test_path.is_file() {
            println!("It's a file");
        }
        
        if let Ok(metadata) = test_path.metadata() {
            println!("File size: {} bytes", metadata.len());
        }
    } else {
        println!("Cargo.toml not found in current directory");
    }
    
    println!();
}

fn directory_operations() {
    println!("7. Directory Operations");
    
    let dir_name = "test_directory";
    
    // Create directory
    match fs::create_dir(dir_name) {
        Ok(()) => println!("✓ Created directory: {}", dir_name),
        Err(e) => println!("Error creating directory: {}", e),
    }
    
    // Create files in directory
    let file1_path = format!("{}/file1.txt", dir_name);
    let file2_path = format!("{}/file2.txt", dir_name);
    
    fs::write(&file1_path, "Content of file 1").expect("Failed to write file1");
    fs::write(&file2_path, "Content of file 2").expect("Failed to write file2");
    
    // List directory contents
    println!("\nDirectory contents:");
    match fs::read_dir(dir_name) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(dir_entry) => {
                        let path = dir_entry.path();
                        let file_name = dir_entry.file_name();
                        
                        println!("  {:?}", file_name);
                        
                        if path.is_file() {
                            if let Ok(size) = dir_entry.metadata().map(|m| m.len()) {
                                println!("    Size: {} bytes", size);
                            }
                        }
                    }
                    Err(e) => println!("Error reading directory entry: {}", e),
                }
            }
        }
        Err(e) => println!("Error reading directory: {}", e),
    }
    
    // Create nested directories
    let nested_path = format!("{}/nested/deep", dir_name);
    match fs::create_dir_all(&nested_path) {
        Ok(()) => println!("✓ Created nested directories: {}", nested_path),
        Err(e) => println!("Error creating nested directories: {}", e),
    }
    
    // Recursive directory listing function
    fn list_dir_recursive(dir: &Path, indent: usize) -> io::Result<()> {
        let entries = fs::read_dir(dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();
            
            println!("{}{:?}", " ".repeat(indent), name);
            
            if path.is_dir() {
                list_dir_recursive(&path, indent + 2)?;
            }
        }
        
        Ok(())
    }
    
    println!("\nRecursive directory listing:");
    if let Err(e) = list_dir_recursive(Path::new(dir_name), 0) {
        println!("Error in recursive listing: {}", e);
    }
    
    // Clean up - remove directory and all contents
    match fs::remove_dir_all(dir_name) {
        Ok(()) => println!("\n✓ Cleaned up: removed {} and all contents", dir_name),
        Err(e) => println!("Error removing directory: {}", e),
    }
    
    // File metadata example
    let current_file = file!(); // Gets the current source file name
    if let Ok(metadata) = fs::metadata(current_file) {
        println!("\nCurrent file metadata:");
        println!("  Size: {} bytes", metadata.len());
        println!("  Is file: {}", metadata.is_file());
        println!("  Is directory: {}", metadata.is_dir());
        
        if let Ok(modified) = metadata.modified() {
            println!("  Modified: {:?}", modified);
        }
    }
    
    println!("\n=== File I/O Complete! ===");
}

// Utility function for safe file operations
fn safe_write_file(filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    file.flush()?;
    Ok(())
}

// Utility function to read file with size limit
fn read_file_limited(filename: &str, max_size: usize) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = fs::metadata(filename)?;
    
    if metadata.len() > max_size as u64 {
        return Err(format!("File too large: {} bytes (max: {})", metadata.len(), max_size).into());
    }
    
    let content = fs::read_to_string(filename)?;
    Ok(content)
}
