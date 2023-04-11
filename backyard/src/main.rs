use crate::garden::vegetable::Cabbage;

pub mod garden;

fn main() {
    let plant = Cabbage {};
    println!("I'm growing {:?}!", plant);
}