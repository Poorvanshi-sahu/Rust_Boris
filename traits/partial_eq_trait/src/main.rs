// PartialEq establishes equality between two values

// #[derive(PartialEq)]

use std::ops::Add;

struct Flight{
    origin: String,
    destination: String,
    time: String
}

impl Flight{
    fn new(origin: &str, destination: &str, time: &str)-> Self{
        Self { 
               origin: origin.to_string(),
               destination: destination.to_string(),
               time: time.to_string()
        }
    }
}

impl PartialEq for Flight {
    fn eq(&self, other: &Self) -> bool {
        self.origin == other.origin && self.destination == other.destination
    }
}

struct Lunch{
    cost : f64
}

impl Add for Lunch{
    type Output = f64;
    
    fn add(self, rhs: Self) -> Self::Output {
        self.cost+rhs.cost
    }
}

fn main() {
    let one = Flight::new("New york", "london", "90:23");
    let two = Flight::new("New york", "london", "20:23");
    let three = Flight::new("New york", "Los angeles", "11:23");

    println!("{}",one == two);
    println!("{}",one.eq(&three));
    println!("{}",one.ne(&three));
    println!("{}",one.ne(&three));

    let one_lunch = Lunch{
        cost: 100.0
    };

    let two_lunch = Lunch{
        cost: 129.0
    };

    println!("{}", one_lunch + two_lunch);
}

// partialOrd trait indicates that a type can be ordered/ sorted
