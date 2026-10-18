

use std::io;

fn main() {
    let mut candidates = Vec::new();

    loop {
        let mut input1 = String::new();
        println!("Enter candidate name (or type 'stop' to finish):");
        io::stdin().read_line(&mut input1).expect("Input error");
        let name = input1.trim();

        if name == "stop" {
            break;
        }
        let mut input2 = String::new();
        println!("Enter years of programming experience:");
        io::stdin().read_line(&mut input2).expect("Not valid float");
        let years:u32 = input2.trim().parse().expect("Not valid number");

        candidates.push((name.to_string(), years));
    }

    let mut highest_name = String::new();
    let mut highest_years = 0;

    for (name, years) in &candidates {
        if *years > highest_years || highest_name.is_empty() {
            highest_years = *years;
            highest_name = name.clone();
        }
    }

    if !highest_name.is_empty() {
        println!("Most experienced candidate: {}", highest_name);
        println!("He has {} years of experience", highest_years);
    } else {
        println!("No candidates were entered.");
    }
}