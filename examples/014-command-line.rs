// 014-command-line.rs
// This example demonstrates building command-line programs in Rust
// Run with: rustc examples/014-command-line.rs -o 014-command-line && ./014-command-line

use std::env;
use std::fs;
use std::io::{self, Write};
use std::process;

fn main() {
    println!("=== Rust Command Line Programs Examples ===\n");

    // Command line arguments
    command_line_args();
    
    // Environment variables
    environment_variables();
    
    // Standard input/output
    standard_io();
    
    // Exit codes
    exit_codes_info();
    
    // Building CLI tools
    cli_tool_examples();
    
    // Configuration and argument parsing
    config_parsing_info();
}

fn command_line_args() {
    println!("1. Command Line Arguments");
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    println!("Program name: {}", args[0]);
    println!("Number of arguments: {}", args.len());
    
    if args.len() > 1 {
        println!("Arguments passed:");
        for (i, arg) in args.iter().enumerate().skip(1) {
            println!("  Arg {}: {}", i, arg);
        }
    } else {
        println!("No arguments passed. Try running with some arguments!");
        println!("Example: ./014-command-line hello world 123");
    }
    
    // Simple argument processing
    match args.len() {
        1 => println!("Usage: {} <command> [args...]", args[0]),
        2 => {
            match args[1].as_str() {
                "help" => println!("This is a help message"),
                "version" => println!("Version 1.0.0"),
                "greet" => println!("Hello! (try: {} greet <name>)", args[0]),
                _ => println!("Unknown command: {}", args[1]),
            }
        }
        3 => {
            if args[1] == "greet" {
                println!("Hello, {}!", args[2]);
            } else {
                println!("Unknown command with 2 args: {} {}", args[1], args[2]);
            }
        }
        _ => println!("Too many arguments"),
    }
    
    println!();
}

fn environment_variables() {
    println!("2. Environment Variables");
    
    // Reading environment variables
    match env::var("USER") {
        Ok(user) => println!("Current user: {}", user),
        Err(_) => {
            // Try Windows equivalent
            match env::var("USERNAME") {
                Ok(user) => println!("Current user: {}", user),
                Err(_) => println!("Could not determine username"),
            }
        }
    }
    
    // Get PATH variable (usually very long)
    match env::var("PATH") {
        Ok(path) => {
            println!("PATH contains {} characters", path.len());
            let paths: Vec<&str> = path.split(':').collect(); // Unix
            let paths: Vec<&str> = if paths.len() == 1 {
                path.split(';').collect() // Windows
            } else {
                paths
            };
            println!("Number of paths in PATH: {}", paths.len());
            
            // Show first few paths
            println!("First few paths:");
            for (i, path) in paths.iter().take(3).enumerate() {
                println!("  {}: {}", i + 1, path);
            }
            if paths.len() > 3 {
                println!("  ... and {} more", paths.len() - 3);
            }
        }
        Err(e) => println!("Could not read PATH: {}", e),
    }
    
    // Setting environment variables (for child processes)
    env::set_var("RUST_EXAMPLE", "Hello from Rust!");
    match env::var("RUST_EXAMPLE") {
        Ok(value) => println!("Set RUST_EXAMPLE to: {}", value),
        Err(_) => println!("Failed to set environment variable"),
    }
    
    // Iterate over all environment variables (showing just a few)
    println!("\nSome environment variables:");
    let mut count = 0;
    for (key, value) in env::vars() {
        if count < 5 && !key.contains("PATH") && value.len() < 50 {
            println!("  {} = {}", key, value);
            count += 1;
        }
    }
    
    println!();
}

fn standard_io() {
    println!("3. Standard Input/Output");
    
    // Standard output (stdout)
    println!("This goes to stdout");
    print!("This also goes to stdout (no newline)");
    println!(" - and this continues the line");
    
    // Standard error (stderr)
    eprintln!("This goes to stderr");
    
    // Flushing output
    print!("Buffered output...");
    io::stdout().flush().expect("Failed to flush stdout");
    println!(" flushed!");
    
    // Reading from stdin (demonstrative - would need actual input)
    println!("\nReading from stdin (simulated):");
    println!("In a real program, you would use:");
    println!("  let mut input = String::new();");
    println!("  io::stdin().read_line(&mut input).expect(\"Failed to read line\");");
    println!("  println!(\"You entered: {{}}\", input.trim());");
    
    // Demonstrate different ways to handle input
    fn simulate_user_input() -> String {
        String::from("simulated user input")
    }
    
    let input = simulate_user_input();
    println!("Simulated input: '{}'", input);
    
    // Input validation example
    fn parse_number(input: &str) -> Result<i32, std::num::ParseIntError> {
        input.trim().parse()
    }
    
    let number_input = "42";
    match parse_number(number_input) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse '{}': {}", number_input, e),
    }
    
    let invalid_input = "not a number";
    match parse_number(invalid_input) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse '{}': {}", invalid_input, e),
    }
    
    println!();
}

fn exit_codes_info() {
    println!("4. Exit Codes and Error Handling");
    
    println!("Exit codes indicate program success/failure:");
    println!("  0 = Success");
    println!("  1-255 = Various error conditions");
    println!();
    
    println!("Ways to exit a program:");
    println!("  std::process::exit(code) - Immediate exit");
    println!("  return from main() - Normal exit");
    println!("  panic!() - Abnormal termination");
    println!();
    
    // Demonstrate exit conditions (without actually exiting)
    fn check_file_exists(filename: &str) -> Result<(), i32> {
        if filename.is_empty() {
            println!("Error: No filename provided");
            return Err(1); // Error code 1
        }
        
        if !std::path::Path::new(filename).exists() {
            println!("Error: File '{}' not found", filename);
            return Err(2); // Error code 2
        }
        
        println!("File '{}' exists", filename);
        Ok(())
    }
    
    // Test the function
    match check_file_exists("") {
        Ok(()) => println!("Success!"),
        Err(code) => println!("Would exit with code: {}", code),
    }
    
    match check_file_exists("nonexistent.txt") {
        Ok(()) => println!("Success!"),
        Err(code) => println!("Would exit with code: {}", code),
    }
    
    // Check if this source file exists
    let current_file = file!();
    match check_file_exists(current_file) {
        Ok(()) => println!("Success!"),
        Err(code) => println!("Would exit with code: {}", code),
    }
    
    println!();
}

fn cli_tool_examples() {
    println!("5. Building CLI Tools");
    
    // Simple file operations tool
    fn file_tool(command: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            "read" => {
                let contents = fs::read_to_string(filename)?;
                println!("File contents:\n{}", contents);
            }
            "size" => {
                let metadata = fs::metadata(filename)?;
                println!("File size: {} bytes", metadata.len());
            }
            "exists" => {
                if std::path::Path::new(filename).exists() {
                    println!("File '{}' exists", filename);
                } else {
                    println!("File '{}' does not exist", filename);
                }
            }
            _ => {
                return Err(format!("Unknown command: {}", command).into());
            }
        }
        Ok(())
    }
    
    // Demonstrate the file tool
    println!("File tool example:");
    
    // Create a test file
    let test_file = "cli_test.txt";
    let test_content = "This is a test file for CLI demonstration.";
    fs::write(test_file, test_content).expect("Failed to create test file");
    
    // Use the file tool
    if let Err(e) = file_tool("exists", test_file) {
        println!("Error: {}", e);
    }
    
    if let Err(e) = file_tool("size", test_file) {
        println!("Error: {}", e);
    }
    
    if let Err(e) = file_tool("read", test_file) {
        println!("Error: {}", e);
    }
    
    // Clean up
    let _ = fs::remove_file(test_file);
    
    // Text processing tool example
    println!("\nText processing tool example:");
    
    fn text_tool(operation: &str, text: &str) -> String {
        match operation {
            "count" => format!("Characters: {}, Words: {}, Lines: {}", 
                text.len(), 
                text.split_whitespace().count(),
                text.lines().count()
            ),
            "upper" => text.to_uppercase(),
            "lower" => text.to_lowercase(),
            "reverse" => text.chars().rev().collect(),
            "sort-lines" => {
                let mut lines: Vec<&str> = text.lines().collect();
                lines.sort();
                lines.join("\n")
            }
            _ => format!("Unknown operation: {}", operation),
        }
    }
    
    let sample_text = "Hello, World!\nThis is Rust.\nCommand line tools are fun!";
    
    println!("Original text:\n{}\n", sample_text);
    println!("Count: {}", text_tool("count", sample_text));
    println!("Upper: {}", text_tool("upper", sample_text));
    println!("Reverse: {}", text_tool("reverse", "Hello"));
    
    println!();
}

fn config_parsing_info() {
    println!("6. Configuration and Argument Parsing");
    
    println!("For production CLI tools, consider using:");
    println!("  - clap crate: Full-featured argument parsing");
    println!("  - structopt crate: Derive-based argument parsing");
    println!("  - argh crate: Lightweight argument parsing");
    println!();
    
    println!("Example Cargo.toml for CLI tool:");
    println!("  [dependencies]");
    println!("  clap = {{ version = \"4.0\", features = [\"derive\"] }}");
    println!("  serde = {{ version = \"1.0\", features = [\"derive\"] }}");
    println!("  toml = \"0.5\"");
    println!();
    
    // Simple config struct example
    #[derive(Debug)]
    struct Config {
        verbose: bool,
        output_file: Option<String>,
        input_files: Vec<String>,
    }
    
    impl Config {
        fn from_args(args: Vec<String>) -> Result<Config, String> {
            let mut config = Config {
                verbose: false,
                output_file: None,
                input_files: Vec::new(),
            };
            
            let mut i = 1; // Skip program name
            while i < args.len() {
                match args[i].as_str() {
                    "-v" | "--verbose" => config.verbose = true,
                    "-o" | "--output" => {
                        i += 1;
                        if i >= args.len() {
                            return Err("Missing output filename".to_string());
                        }
                        config.output_file = Some(args[i].clone());
                    }
                    arg if arg.starts_with('-') => {
                        return Err(format!("Unknown option: {}", arg));
                    }
                    _ => config.input_files.push(args[i].clone()),
                }
                i += 1;
            }
            
            Ok(config)
        }
    }
    
    // Example usage
    let example_args = vec![
        "program".to_string(),
        "--verbose".to_string(),
        "-o".to_string(),
        "output.txt".to_string(),
        "input1.txt".to_string(),
        "input2.txt".to_string(),
    ];
    
    match Config::from_args(example_args) {
        Ok(config) => {
            println!("Parsed configuration:");
            println!("  Verbose: {}", config.verbose);
            println!("  Output file: {:?}", config.output_file);
            println!("  Input files: {:?}", config.input_files);
        }
        Err(e) => println!("Error parsing config: {}", e),
    }
    
    println!("\nBest practices for CLI tools:");
    println!("  ✓ Provide --help and --version options");
    println!("  ✓ Use meaningful exit codes");
    println!("  ✓ Support both short (-v) and long (--verbose) options");
    println!("  ✓ Validate input early");
    println!("  ✓ Provide clear error messages");
    println!("  ✓ Support configuration files");
    println!("  ✓ Use colors for output (with option to disable)");
    println!("  ✓ Show progress for long-running operations");
    
    println!("\n=== Command Line Programs Complete! ===");
}

// Example of a complete mini CLI tool
struct MiniTool {
    verbose: bool,
}

impl MiniTool {
    fn new(verbose: bool) -> Self {
        MiniTool { verbose }
    }
    
    fn log(&self, message: &str) {
        if self.verbose {
            eprintln!("[INFO] {}", message);
        }
    }
    
    fn run(&self, command: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
        self.log(&format!("Running command: {} with {} args", command, args.len()));
        
        match command {
            "echo" => {
                println!("{}", args.join(" "));
            }
            "count" => {
                let text = args.join(" ");
                let char_count = text.chars().count();
                let word_count = text.split_whitespace().count();
                println!("Characters: {}, Words: {}", char_count, word_count);
            }
            "help" => {
                println!("Mini Tool - Available commands:");
                println!("  echo <text>  - Echo the text");
                println!("  count <text> - Count characters and words");
                println!("  help         - Show this help");
            }
            _ => {
                return Err(format!("Unknown command: {}", command).into());
            }
        }
        
        Ok(())
    }
}

// Function to demonstrate the mini tool
fn _demo_mini_tool() {
    let tool = MiniTool::new(true);
    
    let _ = tool.run("echo", &["Hello".to_string(), "World".to_string()]);
    let _ = tool.run("count", &["Hello Rust World".to_string()]);
    let _ = tool.run("help", &[]);
}
