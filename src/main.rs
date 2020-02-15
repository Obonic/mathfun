use std::io;
use std::{i32};


fn main() {
    // Request for input number 1
    println!("Please enter a number: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to readline");
  
    //  Request for input number 2
    println!("Please enter another number: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to readline");
  
    // Convert string to integer and trim that shit.
    let input1 = input1.trim();
    let input2 = input2.trim();
    let x: i32 = input1.parse().ok().expect("This is not a number, Enter a number.");
    let y: i32 = input2.parse().ok().expect("This is not a number, Enter a number.");
  
    // Basic operations
    println!();
    println!("{} + {} = {}", input1, input2, x + y);
    println!();
    println!("{} - {} = {}", input1, input2, x - y);
    println!();
    println!("{} * {} = {}", input1, input2, x * y);
    println!();
    println!("{} / {} = {}", input1, input2, x / y);
    println!();
    println!("{} % {} = {}", input1, input2, x % y);
    println!();
}