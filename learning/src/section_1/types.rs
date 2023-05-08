// We put the struct at the top, outside of the fn main() scope so that way it can be used in any function in the program
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn types_showcase() {
    println!("-=| Types Showcase |=-\n");
    let x = 5; // This is a static variable bc it does not have the 'mut' keyword
    let mut y = 5; // This is a dynamic variable bc it has the 'mut' keyword
    y += 3;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);

    let num : u32 = 5; // This is a variable that will only be able to be  given an unsigned int32 value;
    println!("The value of num is: {}", num);
    let is_bigger = false;
    println!("The value of isBigger is: {}", is_bigger);

    // This is a struct, and we can create them essentially like mini classes, but idk about putting functions in them yet.
    let user1 = User {
        username: String::from("user1"), // To convert a string literal to a string, we use the String::from() function
        email: String::from("user1@gmail.com"),
        sign_in_count: 1,
        active: true};
    
    check_user(user1);
    println!("The value of 5/2 is: {}\n-=-=-=-=-=-=-=-=-=-=", divide_by(5, 2));
}



// This is a void function because it does not have a "-> <DATA>" after the function name
fn check_user(user: User) {
    println!("The value of user1 is: {} | {} | {} | {}", user.username, user.email, user.sign_in_count, user.active);
}

// This is a function that returns a u32 value because it has a "-> u32" after the function name
fn divide_by(x: u32, y: u32) -> u32 {
    return x / y;
}