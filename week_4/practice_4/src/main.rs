// program to determine age pass

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("What is your name?");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let name = input1.trim();

    println!("How old are you?");
    io::stdin().read_line(&mut input2).expect("Not valid string");
    let age:u8 = input2.trim().parse().expect("Not valid number");

    if age >= 18 {println!("Welcome {},hope you have a great time",name );}
    else{println!("Sorry {}, but you are too young to be granted access",name);}

}
