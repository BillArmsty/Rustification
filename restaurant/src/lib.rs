mod front_of_house {
   pub  mod hosting {
       pub fn add_to_waitlist() {}
       
    }

}
//Adding use and a path in a scope is similar to creating a symbolic link in the filesystem. 

use crate::front_of_house::hosting::add_to_waitlist;
//use only creates the shortcut for the particular scope in which the use occurs. 
// mod customer {
//     pub fn eat_at_restaurant(){
//         add_to_waitlist();
//     }
// }



//fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // We can use super to go to the parent module of back_of_house, which in this case is crate, the root. 
       // super::deliver_order();
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }


    //If we make an enum public, all of its variants are then public. We only need the pub before the enum keyword
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    add_to_waitlist();
    //Absolute path is the full path starting from a crate root
    //crate::front_of_house::hosting::add_to_waitlist();

    //Relative path  starts from the current module and uses self, super, or an identifier in the current module
    //front_of_house::hosting::add_to_waitlist();

    //Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    //Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}