use rayon::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{ thread_rng, Rng };

use std::path::Path;
use std::fs::create_dir_all;

use glob::{ glob_with, MatchOptions };
use image::{ FilterType, ImageError };

// //Mutate Elements of an Array in Parallel
fn main() {
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9];
    arr.par_iter_mut().for_each(|p| {
        *p -= 1;
    });
    println!("{:?}", arr);
}

// //Test in parallel if any or all elements of a collection match a given predicate

fn main() {
    let mut vec = vec![2, 4, 6, 8];

    assert!(!vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(!vec.par_iter().any(|n| n > &8));
    assert!(vec.par_iter().all(|n| n <= &8));

    vec.push(9);

    assert!(vec.par_iter().any(|n| (*n % 2) != 0));
    assert!(!vec.par_iter().all(|n| (*n % 2) == 0));
    assert!(vec.par_iter().any(|n| n > &8));
    assert!(!vec.par_iter().all(|n| n <= &8));

    println!("{:?}", vec);
}

// //Search items using given predicate in parallel

fn main() {
    let v = vec![6, 2, 1, 9, 3, 8, 11];

    let f1 = v.par_iter().find_any(|&&x| x == 9);
    let f2 = v.par_iter().find_any(|&&x| x % 2 == 0 && x > 6);
    let f3 = v.par_iter().find_any(|&&x| x > 8);

    assert_eq!(f1, Some(&9));
    assert_eq!(f2, Some(&8));
    assert_eq!(f3, Some(&9));

    println!("{:?}", v);
    println!("{:?}", f1);
}

// Sort a vector in Parallel

fn main() {
    let mut vec = vec![String::new(); 100_000];

    //populate the vector with random strings
    vec.par_iter_mut().for_each(|p| {
        let mut rng = thread_rng();
        *p = (0..5).map(|_| rng.sample(Alphanumeric)).collect::<String>() as String;
    });
    vec.par_sort_unstable();
    println!("{:?}", vec);
}

// Map-Reduce in Parallel

struct Person {
    age: u32,
}

fn main() {
    let v: Vec<Person> = vec![
        Person { age: 23 },
        Person { age: 19 },
        Person { age: 42 },
        Person { age: 17 },
        Person { age: 17 },
        Person { age: 31 },
        Person { age: 30 }
    ];

    let num_over_30 = v
        .par_iter()
        .filter(|&x| x.age > 30)
        .count() as f32;
    let sum_over_30 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .reduce(
            || 0,
            |x, y| x + y
        );

    let alt_sum_30: u32 = v
        .par_iter()
        .map(|x| x.age)
        .filter(|&x| x > 30)
        .sum();

    let avg_over_30 = (sum_over_30 as f32) / num_over_30;
    let alt_avg_over_30 = (alt_sum_30 as f32) / num_over_30;

    assert!((avg_over_30 - alt_avg_over_30).abs() < 0.01);
    println!("Average age of people over 30 = {}", avg_over_30);
}

//Generate JPG thumbnails in parallel

pub mod errors {
    error_chain! {
        foreign_links {
            Glob(::glob::GlobError);
            Image(::image::ImageError);
            Io(::std::io::Error);
        }
    }
}

use errors::*;
use error_chain::error_chain;
use error_chain::ChainedError;



fn main() -> Result<()> {
    let options: MatchOptions = Default::default();
    let files: Vec<_> = glob_with("*.jpg", options)?
        .filter_map(|x| x.ok())
        .collect();

    if files.len() == 0 {
        error_chain::bail!("No .jpg files found in current directory");
    }

    let thumb_dir = "thumbnails";
    create_dir_all(thumb_dir)?;

    println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

    let image_failures: Vec<_> = files
        .par_iter()
        .map(|path| {
            make_thumbnail(path, thumb_dir, 300).map_err(|e|
                e.chain_err(|| path.display().to_string())
            )
        })
        .filter_map(|x| x.err())
        .collect();

    image_failures.iter().for_each(|x| println!("{}", x.display_chain()));

    println!("{} thumbnails created", files.len() - image_failures.len());

    Ok(())
}

fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
    where PA: AsRef<Path>, PB: AsRef<Path>
{
    let img = image::open(original.as_ref())?;
    let file_path = thumb_dir.as_ref().join(original);

    Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest).save(file_path)?)
}
