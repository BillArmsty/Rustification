struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    // struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // };
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };

    // user1.email = String::from("anotheremail@example.com");


    // println!("The email of user1 is: {}", user1.email);

    

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@email.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    //  The above code can be written as follows:
    
    // let user2 = User {
    //     email: String::from("another@email.com"),
    //     ..user1
    // };

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is: {}", area(&rect2));
   

  
} 

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


    
   



