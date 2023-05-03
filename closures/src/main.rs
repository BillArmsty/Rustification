use std::thread;
#[derive(Debug, PartialEq, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => {
                    num_red += 1;
                }
                ShirtColor::Blue => {
                    num_blue += 1;
                }
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}
fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);
    //Adding optional type annotations of the parameter and return value types in the closure

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // //Attempting to call a closure whose types are inferred with two different types

    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    //  Defining and calling a closure that captures an immutable reference

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Defining and calling a closure that captures a mutable reference

    let mut lists = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || lists.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);

    //Using move to force the closure for the thread to take ownership of list
    let liist = vec![1, 2, 3];
    println!("Before defining closure: {:?}", liist);

    thread
        ::spawn(move || println!("From closure: {:?}", liist))
        .join()
        .unwrap();
    Using;
    sort_by_key;
    to;
    order;
    rectangles;
    by;
    width;
    let mut lisst = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    //Using an FnMut closure with sort_by_key

    let mut num_sort_operations = 0;
    lisst.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_opertions} operations", lisst)
}