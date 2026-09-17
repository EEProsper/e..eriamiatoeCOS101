//height checker

use std::io;

fn main() {
    let mut input1 = String::new();

    println!("\n How tall are you?(in centimeters)");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let h:f32 = input1.trim().parse().expect("Not a valid integer");

    if h >= 150.0 && h <= 170.0 {println!("You have an average height");}
    else if h > 170.0 && h <= 195.0 {println!("You are tall");}
    else if h < 150.0 && h > 100.0 {println!("You are short");}
    else {println!("YOUR HEIGHT IS ABNORMAL");}
    

}
