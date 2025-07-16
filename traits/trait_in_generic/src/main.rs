use std::fmt::{Display, Formatter, Result};

enum AppleType{
    RedDelicious,
    GrannySmith
}

impl Display for AppleType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AppleType::RedDelicious => write!(f, "Red delicious"),
            AppleType::GrannySmith => write!(f, "Granny smith")
        }
    }
}

struct Apple{
    kind: AppleType,
    price: f64
}

impl Display for Apple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.price, self.kind)
    }
}

#[derive(Debug, Clone)]
struct book{
    size: u32,
    page: u32,
    time: u32
}

impl Copy for book {}

fn main() {
    // let fruit = Apple{
    //     // kind: AppleType::RedDelicious,
    //     kind: AppleType::GrannySmith,
    //     price: 123.0,
    // };

    // println!("{}", fruit);

    let new_book = book{
        size: 12,
        page: 34,
        time: 2
    };

    let other_book = new_book;

    println!("{:?}", new_book);
}
