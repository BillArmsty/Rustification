fn main() {
    //Creating a new, empty String

    let mut s = String::new();

    //Using the to_string method to create a String from a string literal

    let data = "initial contents";

    let s = data.to_string();

    //The method also works on a literal directly:
    let s = "initial contents".to_string();

    //Using the String::from function to create a String from a string literal

    let s = String::from("initial contents");

    // Appending a string slice to a String using the push_str method

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    //Using a string slice after appending its contents to a String

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    //Adding one character to a String value using push

    let mut d = String::from("lo");
    d.push('l');
    println!("{}", d);

    //Using the + operator to combine two String values into a new String value

    let m1 = String::from("Hello, ");
    let m2 = String::from("world!");
    let m3 = m1 + &m2; // note m1 has been moved here and can no longer be used
    println!("{}", m3);

    //For more complicated string combining, we can instead use the format! macro

    let v1 = String::from("tic");
    let v2 = String::from("tac");
    let v3 = String::from("toe");

    //The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.

    let v = format!("{v1}-{v2}-{v3}");

    //For individual Unicode scalar values, use the chars method.
    //It separates out and returns two values of type char

    for c in "नमस्ते".chars() {
        println!("{c}");
    }

    //For individual bytes, use the bytes method.

    for b in "नमस्ते".bytes() {
        println!("{b}");
    }
}
