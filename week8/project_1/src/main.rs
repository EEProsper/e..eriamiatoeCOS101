

use std::io;

fn main() {
    println!("Input your full name");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let fname= input1.trim();

    println!("Hello {},please note that you have to type all responses in lowercase letters",fname);
     let levels = vec![
        ("office admin", vec![(1, 2, "intern"),(3, 5, "administrator"),(6, 8, "senior administrator"),(8, 10, "office manager"),(10, 13, "director")]),
        ("academic", vec![(3, 5, "research assistant"),(6, 8, "phd candidate"),(8, 10, "post-Doc researcher"),(10, 13, "senior lecturer")]),
        ("lawyer", vec![(1, 2, "paralegal"),(3, 5, "junior associate"),(5, 8, "associate"),(8, 10, "senior associate 1-2"),(10, 13, "senior associate 3-4")]),
        ("teacher", vec![(1, 2, "placement"),(3, 5, "classroom teacher"),(5, 8, "senior teacher"),(8, 10, "leading teacher"),(10, 13, "deputy principal")]),
    ];

    
    println!("APS LEVEL CHECKER");
    println!("Categories: office admin, academic, lawyer, teacher");

    println!("Enter staff category:");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let category = input2.trim();

    println!("Enter years of experience:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not valid float");
    let years: u32 = input3.trim().parse().expect("Not valid number");

    for (name, positions) in &levels {
        if *name == category {
            for (min, max, title) in positions {
                if years >= *min && years <= *max {
                    println!("Staff Category: {}", name);
                    println!("Years of Experience: {}", years);
                    println!("Staff Position: {}", title);
                }
            }
        }
    }
}