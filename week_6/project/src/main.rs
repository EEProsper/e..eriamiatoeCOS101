
use std::io;

fn main() {
    println!("Please input your name");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let name = input1.trim();

    println!("Hello {}, Welcome to the restaurant",name );
    println!("Please input the first thing you would like to order from the menu");
    println!("FOOD ITEM => PRICE");
    println!(" Poundo yam & Edinkaiko soup => 3,200");
    println!(" Fried Rice & Chicken => 3,000");
    println!(" Amala & Ewedu soup => 2,500");
    println!(" Eba & Egusi soup => 2,000");
    println!(" White Rice & stew => 2,500" );
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let order1 = input2.trim();

    println!("How many portions?");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Not valid float");
    let portion:f32 = input3.trim().parse().expect("Not valid number");

    let price1:f32;
    if order1 == "Poundo yam & Edinkaiko soup"{
        price1 = 3_200.00 * portion;
    }
    else if order1 == "Fried Rice & Chicken"{
        price1 = 3_000.00 * portion;
    }
    else if order1 == "Amala & Ewedu soup"{
        price1 = 2_500.00 * portion;
    }
    else if order1 == "Eba & Egusi soup"{
        price1 = 2_000.00 * portion;
    }
    else if order1 == "White Rice & stew"{
        price1 = 2_500.00 * portion;
    }
    else{
        println!("Sorry,we don't have this,i suggest u check your spelling according to our menu");
        price1 = 0.0
    }
    let mut total:f32 = price1;
    loop{
        println!("Would you like to order anything else?");
        println!("Y/N");
        let mut input4 = String::new();
        io::stdin().read_line(&mut input4).expect("Failed to read input");
        let res = input4.trim();

        if res == "Y"{
            println!("What else would you like to order");
            let mut input5 = String::new();
            io::stdin().read_line(&mut input5).expect("Failed to read input");
            let order2 = input5.trim();

            println!("How many portions?");
            let mut input6 = String::new();
            io::stdin().read_line(&mut input6).expect("Not valid float");
            let portion2:f32 = input6.trim().parse().expect("Not valid number");
            
            let price2:f32;
            if order2 == "Poundo yam & Edinkaiko soup"{
                price2 = 3_200.00 * portion2;
                total = total + price2;
            }
            else if order2 == "Fried Rice & Chicken"{
                price2 = 3_000.00 * portion2;
                total = total + price2;
            }
            else if order2 == "Amala & Ewedu soup"{
                price2 = 2_500.00 * portion2;
                total = total + price2;
            }
            else if order2 == "Eba & Egusi soup"{
                price2 = 2_000.00 * portion2;
                total = total + price2;
            }
            else if order2 == "White Rice & stew"{
                price2 = 2_500.00 * portion2;
                total = total + price2;
            }
            else{
                println!("Sorry,we don't have this");
                total = total + 0.0
            }
        }
        else if res == "N"{
            break;
        }
    }   
    println!("Your total is {}",total );
    if total > 10_000.00{
        let discount :f32 = 0.05 * total;
        let new_total:f32 = total - discount;
        println!("You recieved a discount of {}",discount );
        println!("Your new total is {}",new_total );

    }
    println!("Thank you for eating here!");
}
