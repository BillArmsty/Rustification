// //Defining a Summary trait with a default implementation of the summarize method

// pub trait Summary {
//     fn summarize(&self) -> String {
//         String::from("(Read more...)")
//     }
// }

// //Calling the summarize method on an instance of NewsArticle

// let article = NewsArticle {
//     headline: String::from("Penguins win the Stanley Cup Championship!"),
//     location: String::from("Pittsburgh, PA, USA"),
//     author: String::from("Iceburgh"),
//     content: String::from("The Pittsburgh Penguins once again are the best
//     hockey team in the NHL."),
// }

// println!("New article available! {}", article.summarize());


//Conditionally implementing methods on a generic type depending on trait bounds

use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    //Implementing a new method named new that creates a new instance of Pair<T> from two values of type T
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    //Implementing a cmp_display method that only applies to Pair<T> instances where T has the trait bounds of Display and PartialOrd
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}