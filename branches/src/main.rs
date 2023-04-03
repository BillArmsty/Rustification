fn main() {
   let number = 6;
   //One must be explicit and always provide if with a Boolean as its condition.
   

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    //Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable, as shown here:

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    //Returning values from Loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    //Loop Labels to Disambiguate Between Multiple Loops

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    //Conditional Loops with while

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    //Looping Through a Collection with for

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
        
    }

}

