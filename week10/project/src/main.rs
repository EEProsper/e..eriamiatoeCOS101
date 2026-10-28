struct Market{
    price:u32,
    quantity:u32
}

impl Market {
    fn total_cost(&self)->u32 {
        self.price * self.quantity
    }
}

fn main () {
    let small = Market {
        price: 2805000,
        quantity:3
    };
    println!("The total price for one of each laptop is {}\n Quantity:{}\n The total price is {}",small.price,small.quantity,small.total_cost() );
}
