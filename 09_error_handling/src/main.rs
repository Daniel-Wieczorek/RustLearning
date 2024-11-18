use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// using panic to set border req for application:
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Wrong value {value}");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    // panic!("Crash and burn"); => panic mode can be called using macro

    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {error:?}"),
            },
            _other_error => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };

    // choosing panic mode can be done using `expect`
    let _greeting_file = File::open("hello.txt").expect("hello.txt should be included in project!");
}

// propagating errors example using standard approach
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// propagating errors example using rust approach and `?`
fn _read_username_from_file_rusty() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// propagating errors example using most rusty approach and `?`
fn _read_username_from_file_more_rusty() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// propagating errors example using the most most rusty approach and `?`
fn _read_username_from_file_the_most_rusty() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt") // standard library does already everything!
}

// Note: operator `?` can be used only with Option<T> or Result<T,E>.
