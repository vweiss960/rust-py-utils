use std::io::{self, Write};

/// Python-like input function
pub fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}


/// Python-like print macro for multiple arguments
#[macro_export]
macro_rules! print {
    () => {
        ::std::println!();
    };
    ($arg:expr) => {
        ::std::println!("{}", $arg);
    };
    ($arg1:expr, $($args:expr),+) => {
        ::std::print!("{}", $arg1);
        $(
            ::std::print!(" {}", $args);
        )+
        ::std::println!();
    };
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

