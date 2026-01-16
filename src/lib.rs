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
macro_rules! pyprint {
    () => {
        ::std::println!();
    };
    ($arg:expr) => {
        match (&$arg,) {
            (v,) if ::std::any::type_name_of_val(v).contains("Vec") ||
                    ::std::any::type_name_of_val(v).contains("[") => {
                ::std::println!("{:?}", v);
            }
            (v,) => {
                ::std::println!("{}", v);
            }
        }
    };
    ($arg1:expr, $($args:expr),+) => {
        match (&$arg1,) {
            (v,) if ::std::any::type_name_of_val(v).contains("Vec") ||
                    ::std::any::type_name_of_val(v).contains("[") => {
                ::std::print!("{:?}", v);
            }
            (v,) => {
                ::std::print!("{}", v);
            }
        }
        $(
            match (&$args,) {
                (v,) if ::std::any::type_name_of_val(v).contains("Vec") ||
                        ::std::any::type_name_of_val(v).contains("[") => {
                    ::std::print!(" {:?}", v);
                }
                (v,) => {
                    ::std::print!(" {}", v);
                }
            }
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

