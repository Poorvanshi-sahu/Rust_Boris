fn is_item_in_stock(item_is_in_system: bool, item_is_in_stock: bool) -> Option<bool>{
    if item_is_in_stock && item_is_in_system{
        Option::Some(true)
    }
    else if item_is_in_system{
        Option::Some(false)
    }else{
        Option::None
    } 
}

fn main() {
    let availabitlity = is_item_in_stock(true, false);

    match availabitlity{
        // Option::Some(true) => println!("value is: true"),
        // Option::Some(false) => println!("value is: false"),
        // Option::None => println!("value is not present.")

        // top level option variant
        Some(true) => println!("value is: true"),
        Some(false) => println!("value is: false"),
        None => println!("value is not present.")
    }
}
