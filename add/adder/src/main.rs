use add_one;
//Using the add_one library crate from the adder crate
fn main() {
    let num = 10;
    println!("Hello, Stizo! {num} plus one is {}!", add_one::add_one(num));
}
