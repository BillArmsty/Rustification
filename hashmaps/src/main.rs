fn main() {
//Creating a new hash map and inserting some keys and values

use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

//Accessing values in a hash map with the get method

let team_name = String::from("Blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Replacing a value stored with a particular key

scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);

//Using the entry method to only insert if the key does not already have a value

scores.entry(String::from("Yellow")).or_insert(50);
//The or_insert method on Entry is defined to return a mutable reference to the value for the corresponding Entry key if that key exists
scores.entry(String::from("Green")).or_insert(50);

println!("{:?}", scores);


//Updating a value based on the old value

let text = "Hello world wonderful world";

let mut map = HashMap::new();
//Counting occurrences of words using a hash map that stores words and counts
for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map)
}



