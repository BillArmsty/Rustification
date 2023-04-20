//functions that differ only in their names and the types in their signatures

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest

}
// A Point<T, U> generic over two types so that x and y can be values of different types
// struct Point<T> {
//     x: T,
//     y: U,
// }

//Implementing a method named x on the View<M> struct that will return a reference to the x field of type

struct View<M> {
    x: M,
    y: M,
}

impl<M> View<M> {
    fn x(&self) -> &M {
        &self.x
    }
}

//An impl block that only applies to a struct with a particular concrete type for the generic type parameter T

// impl Grid<f32> {
//     fn distance_from_origin(&self) -> f64 {
//         (self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
//     }
// }

//Method that uses generic types different from its struct’s definition

struct Pointer<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Pointer<X1, Y1> {
    fn mixup<U, V>(self, other: Pointer<U, V>) -> Pointer<X1, V> {
        Pointer {
            x: self.x,
            y: other.y,
        }
    }
}


fn main() {

    let p1 = Pointer { x: 5, y: 10.4 };
    let p2 = Pointer { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let m = View { x: 5, y: 10 };

    // let integer = Point { x: 5, y: 10 };
    // let float = Point { x: 1.0, y: 4.0 };
    // let integer_and_float = Point { x: 5, y: 4.0 };

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
   
}
