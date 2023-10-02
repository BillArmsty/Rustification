use std::io;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;

    //Shadowing of a variable by use of let keyword

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //Accessing an array using indexing

    let a = [1, 2, 3, 4, 5];

    println!("Please enter the index of the array you want to access");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please type a number!");

    let element = a[index];

    println!("The value at index {index} is: {element}");

    // Variable scope

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    //VAriables interacting with clone method

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    //Ownership and functions

    let d = String::from("hello"); // d comes into scope

    takes_ownership(d); // d's value moves into the function...
    // ... and so is no longer valid here
    let v = 5; // v comes into scope

    makes_copy(v); // v would move into the function,
    // but i32 is Copy, so it's okay to still
    // use v afterward

    /*
    Return Values and Scope
    */

    let v1 = gives_ownership();

    let v2 = String::from("stizo");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {

    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {

    a_string
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
