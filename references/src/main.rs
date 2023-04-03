fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    //&s1 is a reference to s1 but does not own it.

    println!("The length of '{}' is {}.", s1, len);

    //Mutable references

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s;
    println!("{}", r3);
    
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
}