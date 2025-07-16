#[derive(Debug)]
enum CardSuit{
    Hearts,
    Diamonds,
    Spades,
    Clubs
}

// enum is set of finite possible values

#[derive(Debug)]
enum PaymentMethodType{
    // CreditCard(String, i32, bool),
    CreditCard(String),
    DebitCard(String),
    // Paypal(String, String)
    // Paypal(String)
    Paypal(Credentials)
}

#[derive(Debug)]
struct Credentials{
    username: String,
    password: String
}

#[derive(Debug)]
enum RestaurantItem{
    Burrito{meat: Meat, beans: Beans},
    Bowl(Meat),
    VeganPlate
}

#[derive(Debug)]
enum Meat{
    Chicken,
    Steak
}

#[derive(Debug)]
enum Beans {
    Pinto,
    Black
}

fn main() {
    let first_card = CardSuit::Hearts;
    let mut second_card = CardSuit::Spades;

    second_card = CardSuit::Clubs;

    println!("{:?} {:?}", first_card, second_card);

    let visa =  PaymentMethodType::CreditCard(String::from("Yes working credit card"));
    let masterCard = PaymentMethodType::DebitCard(String::from("Yes working debit card"));

    println!("{:?}", visa);
    println!("{:?}", masterCard);

    let paypalCredentials = Credentials{
        username: String::from("bob@gmail.com"),
        password: String::from("password")
    };

    let paypal = PaymentMethodType::Paypal(paypalCredentials);

    println!("{:?}", paypal);


    //nested enum
    let nested_one = RestaurantItem::Burrito{meat: Meat::Chicken, beans: Beans::Black};
    let nested_two = RestaurantItem::Bowl(Meat::Chicken);

    println!("nested_one: {:?}", nested_one);
    println!("nested_two: {:?}", nested_two);
}
