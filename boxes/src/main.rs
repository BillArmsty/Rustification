use crate::List::{ Cons, Nil };
use std::rc::Rc;

//Defining an enum to represent a cons list data structure of i32 values
// enum List {
//     Cons(i32, List),
//     Nil,
// }

//Definition of List enum that uses Box<T> to have a known size

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

//Definition of List enum that uses Rc<T> to have multiple owners

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

enum Message {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    //Storing an i32 value on the heap using a box
    //    let b = Box::new(5);
    //     println!("b = {}", b);

    //Using the List enum to store the list 1, 2, 3
    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
