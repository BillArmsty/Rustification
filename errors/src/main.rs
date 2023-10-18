use std::fs::File;
use std::io::{ ErrorKind, Read, self };

fn main() {
    //Attempting to access an element beyond the end of a vector, which will cause a call to panic!

    let v = vec![1, 2, 3];
    v[99];
    // Handling different kinds of errors in different way

    let greeting_file_result = File::open("hello.txt");

    //Using a match expression to handle the Result variants that might be returned

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) =>
            match error.kind() {
                ErrorKind::NotFound =>
                    match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    }
                other_error => { panic!("Problem opening the file: {:?}", other_error) }
            }
    };

    //Alternative to using match with Result<T, E>

    let greeting_filee = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //expect method lets us also choose the panic! error message.
    //Using expect instead of unwrap and providing good error messages can convey your intent and make tracking down the source of a panic easier

    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");

    // A function that returns errors to the calling code using match

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => {
                return Err(e);
            }
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    //A function that returns errors to the calling code using the ? operator

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // Chaining method calls after the ? operator

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }

    //Using fs::read_to_string instead of opening and then reading the file

    fn read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
}

//Changing main to return Result<(), E> allows the use of the ? operator on Result values in main

use std::fs::File;
use ::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
