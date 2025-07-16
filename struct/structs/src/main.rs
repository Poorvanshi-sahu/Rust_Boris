#[derive(Debug)]

struct Coffee{
    price: f64,
    name: String,
    is_hot: bool
}

fn main() {
    struct CoffeeDrink{
        price: f64,
        name: String,
        is_hot: bool
    }

    let mut mocha = CoffeeDrink{
        price: 90.0,
        name: String::from("Mocha"),
        is_hot: true
    };

    println!("{} {} {}", mocha.name, mocha.price, mocha.is_hot);

    let my_fav_coffee = mocha.name;
    println!("{my_fav_coffee}");

    // println!("{}", mocha.name); error as ownership is moved

    // ****************************overwrite struct field
    mocha.name = String::from("newname");

    println!("{}",mocha.name);

    // **************************** create struct in function
    let name = String::from("meri coffee");
    let my_coffee = make_coffee(name, 123.0, false);


    println!("{} {} {}", my_coffee.name, my_coffee.price, my_coffee.is_hot);
    // println!("{}", name);

    // shorthand syntax
    let price = 90.0;
    let name = String::from("random");
    let is_hot = false;

    let mut new_coffee = Coffee{
        price,
        name,
        is_hot
    };

    println!("shorthand: {} {} {}", new_coffee.name, new_coffee.price, new_coffee.is_hot);


    // struct update syntax
    let updated_struct = CoffeeDrink{
        // to avoid passing ownership
        name: mocha.name.clone(),
        ..mocha
    };

    println!("updated_struct: {} {} {}", updated_struct.name, updated_struct.price, updated_struct.is_hot);
    println!("mocha name: {}", mocha.name);


    // passing struct in function
    println!("{}", new_coffee.is_hot);

    drink_coffee(&mut new_coffee);

    println!("{}", new_coffee.is_hot);

    // **************************** debug trait for struct
    let arr = [1,3,4,4];
    // println!("{}", arr);
    println!("{:?}", arr); // debug trait
    println!("{:#?}", arr); // pretty print

}


// **************************** create struct in function
fn make_coffee(name: String, price: f64, is_hot: bool) -> Coffee{
    // Coffee{
    //     name: name,
    //     price: price,
    //     is_hot: is_hot
    // }

    // shorthand syntax
    Coffee{
        name,
        price,
        is_hot
    }
}

fn drink_coffee(coffee: &mut Coffee){
    println!("my coffee name is: {}",coffee.name);
    coffee.is_hot = true
}