use std::io;

fn main() {
    let  x = 5;
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


 
}
