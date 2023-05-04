fn main() {
    //Creating an iterator

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    //Using an iterator in a for loop

    for val in v1_iter {
        println!("Got: {}", val);
    }

    //Calling iterator adaptors map

    let v1: Vec<i32> = vec![1, 2, 3];

    //Using collect method to consume the new iterator and create a vector

    let v2: Vec<_> = v1
        .iter()
        .map(|x| x + 1)
        .collect();

    assert_eq!(v2, vec![2, 3, 4]);

    println!("v2: {:?}", v2);
}