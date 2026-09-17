
use std::io;

fn main() {
    println!("What is the coefficient of x square in your equation?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not valid float");
    let a = input1.trim();

    println!("What is the coefficient of x in your equation?");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not valid float");
    let b = input2.trim();

    println!("What is the value of the number that does not have x in your eqaution?");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not valid float");
    let c = input3.trim();

    let mut d:f32 = (b*b) - (4.0 * a * c);
    if d > 0.0 {let r1: f32 = (-b + ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a);
        let r2: f32 = (-b - ((b * b) - (4.0 * a * c)).sqrt()) / (2.0 * a);
        println!("Your equation has two distinct roots which are {} and {}",r1,r2 );}
    else if d == 0.0 {let r:f32 = (-b) / (2.0 * a) ;
        println!("Your equation has exactly one real root which is {}",r);}
    else if d < 0.0 {println!("Your equation has no real roots, I advice that you recheck the values inputted");}  


}
