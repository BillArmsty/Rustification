fn main() {
    //A match expression with an arm that introduces a shadowed variable y
    let x = Some(5);
    let v = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(v) => println!("Matched, v = {v}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, v = {v}", x);

    //Match multiple patterns using the | syntax,
    let y = 2;
    match y {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //Match ranges of values with ..= syntax
    let z = 5;

    match z {
        1..=10 => println!("one through ten"),
        _ => println!("something different"),
    }
}
