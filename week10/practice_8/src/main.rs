struct Employee {
    ceo:String,
    company:String,
    age:u8,
}
fn main(){
    let emp1 = Employee {
        company:String::from("Microsoft corporations"),
        ceo:String::from("Satya Nadella"),
        age:56,
    };
    let emp2 = Employee{
        company:String::from("Google Inc."),
        age:51,
        ceo:String::from("Sundai Pichai"),
    };
    display (emp1);
    display(emp2);
}

fn display(emp:Employee){
    println!("Name is {}, company is {} and age is {}",emp.ceo,emp.company,emp.age );
}
