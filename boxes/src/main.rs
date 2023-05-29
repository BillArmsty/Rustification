use crate::List::{Cons, Nil};

//Defining an enum to represent a cons list data structure of i32 values
// enum List {
//     Cons(i32, List),
//     Nil,
// }

//Definition of List enum that uses Box<T> to have a known size

enum List {
    Cons(i32, Box<List>),
    Nil,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {

   //Storing an i32 value on the heap using a box
   let b = Box::new(5);
    println!("b = {}", b);


    //Using the List enum to store the list 1, 2, 3
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
   


}
