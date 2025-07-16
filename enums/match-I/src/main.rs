enum OperatingSystem{
    Windows,
    Linux,
    MacOS
}

enum LaundryCycle{
    Cold,
    Hot{temperature: u32},
    Delicate(String)
}

// match IV
enum OnlineOrderStatus{
    Ordered,
    Packed,
    Shipped,
    Delievered
} 

impl OnlineOrderStatus{
    fn check(&self){
        match self{
            OnlineOrderStatus::Ordered => {
                println!("Your ordered has been ordered")
            }
            OnlineOrderStatus::Packed => {
                println!("Your ordered has been packed")
            }
            OnlineOrderStatus::Shipped => {
                println!("Your ordered has been Shipped")
            }
            OnlineOrderStatus::Delievered => {
                println!("Your ordered has been delievered")
            }
            _ => {
                println!("catch all")
            }
        }
    }
}

// match III
impl LaundryCycle{
    fn wash_laundry(&self){
        match self{
            LaundryCycle::Cold => {
                println!("Running the laundry with cold water");
            },
            LaundryCycle::Hot{temperature} => {
                println!("Running the laundry with hot water at temperature of: {}", temperature);
            },
            LaundryCycle::Delicate(fabric) => {
                println!("Running the laundry with delicate water with fabric type: {}",fabric);
            }
        }
    }
}

// if let
enum Milk{
    Lowfat(i32),
    Whole,
    NonDairy{ kind: String},
}

fn main() {
    // let number = 20;

    // let res = match number{
    //     5 => println!("I am five")
    //     10 => println!("I am ten")
    //     _ => println!("I am default")
    // };

    // println!("{:?}", res);

    let my_computer = OperatingSystem::Linux;
    println!("My computer's os is {} years old", year_since_release(my_computer));

    let dad_computer = OperatingSystem::Windows;
    let age = year_since_release(dad_computer);
    println!("My dad's computer is {} years old", age);

    let laundry_delicate = LaundryCycle::Delicate(String::from("silk"));
    let laundry_hot = LaundryCycle::Hot{temperature:32};
    let laundry_cold = LaundryCycle::Cold;

    // wash_laundry(laundry_delicate);
    // wash_laundry(laundry_hot);
    // wash_laundry(laundry_cold);

    laundry_cold.wash_laundry();

    // MATCH IV
    let ordered = OnlineOrderStatus::Ordered;
    let packed = OnlineOrderStatus::Packed;
    let shipped = OnlineOrderStatus::Shipped; 
    let delievered = OnlineOrderStatus::Delievered;

    delievered.check();
    // packed.check();
    // shipped.check();
    // ordered.check();

    // if let
    let my_beverage = Milk::Whole;

    if let Milk::Whole = my_beverage{
        println!("You have whole milk");
    }
}


fn year_since_release(os: OperatingSystem) -> u32{
    match os{
        OperatingSystem::Windows => {
            println!("This is windows os");
            39
        },
        OperatingSystem::Linux => 12,
        OperatingSystem:: MacOS =>24,
    }
}

// fn wash_laundry(cycle: LaundryCycle){
//     match cycle{
//         LaundryCycle::Cold => {
//             println!("Running the laundry with cold water");
//         },
//         LaundryCycle::Hot{temperature} => {
//             println!("Running the laundry with hot water at temperature of: {}", temperature);
//         },
//         LaundryCycle::Delicate(fabric) => {
//             println!("Running the laundry with delicate water with fabric type: {}",fabric);
//         }
//     }
// }