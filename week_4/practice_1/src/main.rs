// rust program to output name and age

use std::io;

fn main() {
    println!("\n Student Integration Management System!" );

    // input name 
    println!("\n What is your name?");
    let mut name = String::new();
       io::stdin()
       .read_line(&mut name)
       .expect("Failed to read your input");
    let name = name.trim(); 
    println!("Your name is {}",name );

    // input age 
    println!("\n How old are you?");
    let mut age = String::new();
       io::stdin()
       .read_line(&mut age)
       .expect("Failed to read your input");
    let age:u8 = age.trim().parse().expect("The Input is not integer");
    println!("Your age is {}",age);

    println!("Your name is {}, and you are {} years old",name,age );
}
