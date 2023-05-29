use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    //Returns a reference to the value we want to access with the * operator
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(f: T) -> MyBox<T> {
        //Takes ownership of x and stores it in a new MyBox instance
        MyBox(f)
    }
}

fn main() {
    //Using dereference operator to follow a reference to i32 value
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Using the dereference operator on a Box<i32>

    let a = 5;
    let b = Box::new(a);

    assert_eq!(5, a);
    assert_eq!(5, *b);
}
