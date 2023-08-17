use rand::Rng;
use rand::distributions::{ Distribution, Uniform, Normal, NormalError };
use rand_distr::{Distribution, Normal, NormalError};
use rand::thread_rng;

fn main() -> Result<(), NormalError> {
    // Generate a random numbers.
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());

    // Generate a random value between 0 and 10.

    let mut rngg = rand::thread_rng();
    println!("Integer: {}", rngg.gen_range(0..10));
    println!("Float: {}", rngg.gen_range(0.0..10.0));

    //Use uniform for a more uniform distribution.

    let mut rnggg = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rnggg);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    // Generate a random value from a Normal distribution.
    let mut rngggg = rand::thread_rng();
    let normal = Normal::new(2.0, 3.0)?;
    let v = normal.sample(&mut rngggg);
    println!("{} is from a N(2, 9) distribution", v);
    Ok(())
}
