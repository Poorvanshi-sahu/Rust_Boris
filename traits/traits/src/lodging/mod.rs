use std::{collections::HashMap, fmt::Debug, ops::Add, str::FromStr};


pub trait Accommodation:Debug {
    fn book(&mut self, name: &str, nights: u32);
}

pub trait Description {
    fn get_description(&self) -> String{
        String::from("Wonderful place to stay")
    }
}

#[derive(Debug)]
pub struct Hotel{
    name: String,
    reservations: HashMap<String, u32>
}

impl Hotel {
    pub fn new(name: &str)-> Self{
        Self{ 
            name: name.to_string(), 
            reservations: HashMap::new() 
        }
    }

    fn summarize(&self)-> String{
        format!("{}: {}", self.name, self.get_description())
    }
}

impl Description for Hotel{}

impl Accommodation for Hotel {
    // will go to default declaration
    // fn get_description(&self)->String {
    //     format!("{} is the pinnacle of luxury.", self.name)
    // }

    fn book(&mut self, name: &str, nights: u32) {
        self.reservations.insert(name.to_string(), nights);
    }
}

#[derive(Debug)]
pub struct AirBnb{
    host: String,
    guests: Vec<(String, u32)>
}

impl AirBnb {
    pub fn new(host: &str)->Self{
        Self{ 
            host: host.to_string(),
            guests: vec![]
        }
    }
}

impl Accommodation for AirBnb {
    fn book(&mut self, name: &str, nights: u32) {
        self.guests.push((name.to_string(), nights));
    }
}

impl Description for AirBnb {
    fn get_description(&self)->String {
        format!("Please enjoy {}'s apartment", self.host)
    }
}

