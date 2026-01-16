use std::io::{self, Write};

/// Python-like input function
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


/// Python-like print with multiple args
pub fn print(args: &[&dyn std::fmt::Display]) {
    for (i, arg) in args.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}", arg);
    }
    println!();
}

/// Trait for Python-like split method
pub trait PySplit {
    fn pysplit(&self, delimiter: &str) -> Vec<String>;
}

impl PySplit for String {
    fn pysplit(&self, delimiter: &str) -> Vec<String> {
        self.split(delimiter)
            .map(|s| s.to_string())
            .collect()
    }
}

impl PySplit for &str {
    fn pysplit(&self, delimiter: &str) -> Vec<String> {
        self.split(delimiter)
            .map(|s| s.to_string())
            .collect()
    }
}

