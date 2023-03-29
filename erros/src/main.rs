use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

// you can also return errors from the main function!
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// example of error delegation - where you return the error to the calling code
fn read_username_from_file() -> Result<String, io::Error> {
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

// same as the above code, but more concise with the ? operator
fn read_username_from_file_concise() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // shortcut to: value if Ok, else return Err
    let mut username = String::new();
    username_file.read_to_string(&mut username)?; // same thing here
    Ok(username)
}

// even more concise - can chain ?
fn read_username_from_file_conciser() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// use fs method instead of File
fn read_username_from_file_concisest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// can also be done with Option values too!
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
