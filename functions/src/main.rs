fn main() {
    // println!("Hello, world!");
    // open_store("narsingi");
    // baking_pizza(4, "paneer");
    // println!("{}", square(5));

    // let e_f = empty_fn();
    // println!("{:?}",e_f);

    // ***************************************** function scope

    let multiplier = 10;

    let scope = {
        let value = 20;
        value * multiplier
        
    };

    println!("scope: {scope}");
    // println!("{}", value);


    // project calls
    apply_to_jobs(35, "Rust Developer");
    let ans = is_even(91);
    println!("ans: {ans}");

    println!("{:?}", alphabets("zebra"));
}

fn open_store(neighborhood: &str){
    println!("Opening my pizza store {neighborhood}");
}

fn baking_pizza(number : i32, topping: &str){
    println!("Baking {number} {topping} pizza");
}

fn _swim_in_profit(){
    println!("So much profit");
}

fn square(number: i32)->i32{
    number*number;
    9
}

fn empty_fn(){}

// ********************************************************project


fn apply_to_jobs(number: i32, title: &str){
    println!("I 'm applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool{
    number % 2 ==0
}

fn alphabets(text: &str) -> (bool, bool){
    let first = text.contains('a');
    let second = text.contains('z');

    (first, second)
}