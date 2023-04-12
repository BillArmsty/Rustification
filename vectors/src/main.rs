fn main() {
    //Creating a new, empty vector to hold values of type i32
    let v: Vec<i32> = Vec::new();
    let m: Vec<i32> = Vec::new();
    let n: Vec<i32> = Vec::new();

    //Creating a new vector with initial values
    let v = vec![1, 2, 3];

    //Updating a vector
    //Create a vector and add elements to it using push method

    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //Reading elements of vectors
    //Using indexing syntax or the get method to access an item in a vector

    let v = vec![1,2,3,4,5];
     
    //Using & and [] gives us a reference to the element at the index value.
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }


    //Iterating over the values in a vector using a for loop
    let  m = vec![100, 32, 57];
    for i in &m {
        println!("{}", i);
    }

    //Iterating over mutable references to elements in a vector
    let mut n = vec![99, 35, 63];
    for i in &mut n {
        //To change the value that the mutable reference refers to, 
        //We have to use the * dereference operator to get to the value in i before we can use the += operator
        *i += 50;
        println!("{}", i);
    }

    //Defining an enum to store values of different types in one vector

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec! [
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ]


}
