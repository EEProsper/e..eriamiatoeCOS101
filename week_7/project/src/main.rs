
use std::io;

fn main() {
    println!("What shape are you working with?(NOTE: Let all inputs be in lowercase)");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let shape = input1.trim();

    println!("What is the unit ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let unit = input2.trim();
    if shape == "trapezium"{
        trapezium(unit);}
    else if shape == "rhombus"{
        rhombus(unit);}
    else if shape == "parallelogram"{
        parallelogram(unit);}
    else if shape == "cube"{
        cube(unit);}
    else if shape == "cylinder"{
        cylinder(unit);}
    else {
        println!("Unsupported shape");
    }

}
fn trapezium(unit:&str){
        println!("What is the value of your first base");
        let mut input3 = String::new();
        io::stdin().read_line(&mut input3).expect("Not valid float");
        let b1:f32 = input3.trim().parse().expect("Not valid number");

        println!("What is the value of your second base");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Not valid float");
        let b2:f32 = input4.trim().parse().expect("Not valid number");

        println!("What is the value of your height");
        let mut input5 = String::new();
        io::stdin().read_line(&mut input5).expect("Not valid float");
        let h:f32 = input5.trim().parse().expect("Not valid number");

        let area:f32 = (h / 2.0) * (b1 + b2);
        println!("Area of your trapezium is {}{}^2",area,unit );
    
}

fn rhombus(unit:&str) {
        println!("What is the value of the first diagonal");
        let mut input6 = String::new();
        io::stdin().read_line(&mut input6).expect("Not valid float");
        let d1:f32 = input6.trim().parse().expect("Not valid number");

        println!("What is the value of the second diagonal");
        let mut input7 = String::new();
        io::stdin().read_line(&mut input7).expect("Not valid float");
        let d2:f32 = input7.trim().parse().expect("Not valid number");

        let ar:f32 = 0.5 * d1 * d2;
        println!("The area of your rhombus is {}{}^2",ar,unit );
    
}

fn parallelogram(unit:&str) {
        println!("What is the value of the base of your parallelogram");
        let mut input8 = String::new();
        io::stdin().read_line(&mut input8).expect("Not valid float");
        let b:f32 = input8.trim().parse().expect("Not valid number");

        println!("What is the altitude of the parallelogram");
        let mut input9 = String::new();
        io::stdin().read_line(&mut input9).expect("Not valid float");
        let a:f32 = input9.trim().parse().expect("Not valid number");

        let are:f32 = a * b;
        println!("The area of your parallelogram is {}{}^2",are,unit );        
    
}

fn cube(unit:&str) {
        println!("Would you like to calculate surface area or volume of your cube");
        let mut input10 = String::new();
        io::stdin().read_line(&mut input10).expect("Failed to read input");
        let res = input10.trim();

        if res == "surface area"{
            println!("What is the value of the side of your cube?");
            let mut input11 = String::new();
            io::stdin().read_line(&mut input11).expect("Not valid float");
            let l:f32 = input11.trim().parse().expect("Not valid number");

            let s_area:f32 = 6.0 * l * l;
            println!("The surface area of your cube is {}{}^2",s_area,unit );

        }
        else if res == "volume"{
           println!("What is the value of the side of your cube?");
           let mut input12 = String::new();
           io::stdin().read_line(&mut input12).expect("Not valid float");
           let side:f32 = input12.trim().parse().expect("Not valid number");

           let vol:f32 = side * side * side;
           println!("The volume of your cube is {}{}^3",vol,unit );
        }
    
}

fn cylinder(unit:&str) {
        println!("Would you like to calculate the volume or surface area of your cylinder");
        let mut input13 = String::new();
        io::stdin().read_line(&mut input13).expect("Failed to read input");
        let resp = input13.trim();

        if resp == "volume"{
            println!("What is the value of the radius of your cylinder?");
            let mut input14 = String::new();
            io::stdin().read_line(&mut input14).expect("Not valid float");
            let r:f32 = input14.trim().parse().expect("Not valid number");

            println!("What is the height of your cylinder?");
            let mut input15 = String::new();
            io::stdin().read_line(&mut input15).expect("Not valid float");
            let height :f32 = input15.trim().parse().expect("Not valid number");

            let volume:f32 = 3.142 * r * r * height;
            println!("The volume of your cylinder is {}{}^3",volume,unit );
        }
        else if resp == "surface area"{
            println!("What is the value of the radius of your cylinder?");
            let mut input16 = String::new();
            io::stdin().read_line(&mut input16).expect("Not valid float");
            let rad:f32 = input16.trim().parse().expect("Not valid number");

            println!("What is the height of your cylinder?");
            let mut input17 = String::new();
            io::stdin().read_line(&mut input17).expect("Not valid float");
            let he:f32 = input17.trim().parse().expect("Not valid number");

            let sa:f32 = (2.0 * 3.142 * rad * he) + (2.0 * 3.142 * rad * rad);
            println!("The surface area of your cylinder is {}{}^2",sa,unit);
        }

    
}
