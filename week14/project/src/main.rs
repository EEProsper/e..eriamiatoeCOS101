use std::io::Read;
use std::io;

fn main(){
    println!("Please identify yourself (Administrator,Project manager,Employee,Customer,Vendor)");
    let mut response = String::new();
    let _ = io::stdin().read_line(&mut response);
    let response = response.trim();

    if response == "Administrator"{
        let mut file = std::fs::File::open("globacom_db.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}",contents );
    }
    else if response == "Project manager"{
        let mut file = std::fs::File::open("project.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}",contents );
    }
    else if response == "Employee"{
        let mut file = std::fs::File::open("staff.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}",contents );
    }
    else if response == "Customer"{
        let mut file = std::fs::File::open("customer.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}",contents );
    }
    else if response == "Vendor"{
        let mut file = std::fs::File::open("dataplan.sql").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{}",contents );
    }
    else {
        println!("INVALID INPUT(CHECK YOUR USE OF CAPITAL AND SMALL LETTERS)");
    }
}
