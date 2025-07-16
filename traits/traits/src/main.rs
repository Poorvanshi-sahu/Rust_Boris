use std::ops::Add;
use std::str::FromStr;

use traits::lodging::{AirBnb, Hotel, Description, Accommodation};

fn main() {
    let mut hotel = Hotel::new("Radission");
    // println!("{}", hotel.get_description());
    // hotel.book("poorvanshi", 30);
    // hotel.book("poorv", 5);
    // println!("{}", hotel.summarize());

    // book_for_one_night(&mut hotel, "non", 21);
    // println!("{:#?}", hotel);

    let mut airbnb = AirBnb::new("Happy homes");
    // println!("{}", airbnb.get_description());
    // airbnb.book("Riu", 12);
    // println!("{:#?}", airbnb);                                          

    // mix_and_match(&mut hotel,&mut airbnb, "owner");

    // println!("{:#?} {:#?}", hotel, airbnb);

    let stays: Vec<&dyn Description> = vec![&airbnb, &hotel];
    println!("{}", stays[1].get_description());

    let mut stays: Vec<&mut dyn Accommodation> = vec![&mut airbnb, &mut hotel];
    stays[1].book("bob", 12);
    println!("{:?}", stays[1]);


    // traits must be in scope to utilize its implementation
    // add is the implementation of Add trait
    let a = 32;
    let b = 12;
    let res = a.add(b);
    println!("{}", res);

    let numeric_count = u64::from_str("53");
    println!("{}", numeric_count.unwrap());
}
