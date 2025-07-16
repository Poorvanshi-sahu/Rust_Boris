// stack is speedy and efficient
// heap is slow but dynamic in nature

use std::{os::unix::process, string};

fn main() {
    println!("Hello, world!");

    // String
    let _food: &str = "Pasta";

    // String is a growable, heap-allocated data structure, it is used when we need a mutable string or a string that can change size.
    // When we are not sure about the length of the string at compile time, we use String.
    // String is mutable

    let text: String = String::new();

    // the clone method is for creating a deep copy of the string.
    // It creates a new instance of the string with the same value as the original string.

    let new_text = text.clone();

    let new_var = String::from("skdjfl");

    println!("{new_var}");

    let candy: String = String::from("kitkat is a chocolate bar");

    println!("here: {text} {candy} {new_text}");

    // push_str

    let mut new_word = String::from("new");

    new_word.push_str("__word");

    // drop is to deallocate the memory of the variable after we can no longer use it.
    drop(new_word);
    // println!("{new_word}");


    // refernces and borrowing 

    let integer = 10;
    let interger_reference: &i32 = &integer;

    // let string_value = String::from("Hello, Rust!");
    // let string_reference: &String = &string_value;

    let string_value = String::new();
    let string_reference: &String = &string_value;

    println!("{string_reference}");

    // deferencing

    println!("derfernced value: {}", *interger_reference);

    let mut word: &str = "word";
    let mut S_word = String::from("I am string type");

    S_word.push_str("I am  new");
    S_word = S_word.replace("I am string type", "New text");

    word = "new word";

    println!("{S_word}");


    // **************************************** Ownership
    let apples = 10;
    print_my_value(apples);
    println!("{apples}");

    // let oranges = String::from("oranges");
    // print_my_value_string(oranges);
    // println!("{oranges}");

    // **************************************** mutable parameters
    println!("********************************************************************mutable parameters");
    let burger = 10;
    change_burger(burger);
    println!("burger after change fn: {burger}");

    // ***************************************** return I
    println!("********************************************************************return I");
    let mut cake = bake_cake();
    println!("{}", cake);

    cake.push_str(" with new string");

    println!("{}", cake);

    // ***************************************** return II
    println!("********************************************************************return II");
    let mut cake = String::new();

    cake = bake_cake_2(cake);
    cake = bake_cake_3(cake);
    cake = bake_cake_4(cake);

    println!("{}", cake);
}

fn print_my_value(value: i32){
    println!("print value is :{value}")
}

fn print_my_value_string(value: String){
    println!("string value is :{value}");
}

fn change_burger(mut burger: i32){
    burger += 10;
    println!("burger in change fn: {burger}");
}


fn bake_cake()-> String{
    let cake = String::from("Chocolate brownie");
    return cake;
}


fn bake_cake_2(mut cake: String)->String{
    cake.push_str("Chocolate");
    cake
}

fn bake_cake_3(mut cake: String)->String{
    cake.push_str("Mousse");
    cake
}

fn bake_cake_4(mut cake: String)->String{
    cake.push_str("Vanilla");
    cake
}