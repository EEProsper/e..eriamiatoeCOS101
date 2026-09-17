// program to calculate area of a triangle with base and height

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the base of your triangle");
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let b:f32 = input1.trim().parse().expect("Not valid integer");

    println!("Enter the height of your triangle");
    io::stdin().read_line(&mut input2).expect("Not valid string");
    let h:f32 = input2.trim().parse().expect("Not valid integer");

    if b > 0.0 && h > 0.0{
      let area:f32 = (b * h)/2.0;
      println!("The area of your triangle is {}",area );
    } else {println!("Your base cannot be a negative number,run the program again to restart");}  


}
