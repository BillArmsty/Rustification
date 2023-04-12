use std::fmt::Result;
use std::io::Result as IoResult;
//To bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
use std::collections::*;

//In the second use statement, we chose the new name IoResult for the std::io::Result type, 
//which won’t conflict with the Result from std::fmt that we’ve also brought into scope.

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}

//Re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
//Making a name available for any code to use from a new scope with pub use
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}