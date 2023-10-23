// syntax of specifying generic type parameters, trait bounds, and lifetimes

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

//A struct that holds a reference, requiring a lifetime annotation

struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
    println!("{}", i.part);

    //Static lifetime
    //Denotes the affected reference can live for the entire duration of the program

    let s: &'static str = "I have a static lifetime.";

    
}
