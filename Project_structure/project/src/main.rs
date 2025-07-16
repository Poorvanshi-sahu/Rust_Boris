mod inventory;
mod stocks;

///It is that comment, visible

// standard library
use std::{
    fmt
};

//************************* (*)glob operator ************************* */
use std::collections::*; // (import everthing in the current namespace, but generally avoid so that there will no implicit name collision)

use fake::{Fake, Faker as f};

use inventory::product::{Item, ProductCategory};
// use inventory::{MANAGER, FLOOR_SPACE};
use inventory::FLOOR_SPACE;
use inventory::MANAGER as New_Manager;

fn main() {
    println!("Hello, world!, Hello {}", inventory::MANAGER);
    println!("{}", stocks::MANAGER);

    println!("{}", FLOOR_SPACE);

    let choose_enum = ProductCategory::Ladders;
    println!("My favourite enum is: {:?}", choose_enum);

    let tall_ladder =  Item{
        name: String::from("abc"),
        category: ProductCategory::Ladders,
        quantity: 100
    };

    println!("{:?}", tall_ladder);

    let item_instance = Item::new(String::from("abc"), ProductCategory::Hammers, 123);
    println!("new instance: {:?}", item_instance);

    let fake_item: Item = f.fake();
    println!("{:?}", fake_item);
}



