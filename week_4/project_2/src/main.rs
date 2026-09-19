

use std::io;

fn main() {
    println!("Input your name");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Not valid string");
    let n = input1.trim();
    
    println!("Hello {},Please input your age",n );
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Not valid float");
    let age:u32 = input2.trim().parse().expect("Not valid number");

    println!("Are you experienced or not?,Please input experienced or not experienced");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not valid string");
    let exp = input3.trim();

    if age >= 40 && exp == "experienced"{
        println!("Your annual incentive is #1,560,000" );
    }
    else if age >= 30 && age <=39 && exp == "experienced"{
        println!("Your annual incentive is #1,480,000");
    }
    else if age < 29 && exp == "experienced"{
        println!("Your annual incentive is #1,300,000");
    }
    else if exp == "not experienced"{
        println!("Your annual incentive is #100,000");
    }
}
