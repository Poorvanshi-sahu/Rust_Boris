/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.
 
Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.
 
Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the `!` symbol and return a vector of the string
slices. Invoke the function in `main`.
 
Example:
elements("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]
 
Define a `get_identity` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the `expect`
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `get_identity` function in `main`, and output the
returned String.
 
Example:
fn main() {
  let name = get_identity();
   println!("{name}"); // Bill Murray
}
*/

use std::{fmt::format, io};

fn main() {
    let mut val = String::from("hello world ");
    make_money(&mut val);

    println!("{}", val);

    let s = "poorvanshi   ";
    let res = trim_and_capitalize(s);
    println!("{}", res);

    println!("{:?}", elements("Gold!Silver!Platinum"));

    let name = get_identity();
    println!("{}", name);
}

fn make_money(val: &mut String){
    val.push_str("$$$");
}

fn trim_and_capitalize(val: &str)-> String{
    val.trim().to_uppercase() // uppercase returns string
}

fn elements(val: &str)-> Vec<&str>{
    val.split("!").collect::<Vec<&str>>()
}

fn get_identity()-> String{
    let mut first_name = String::new();
    let mut last_name = String::new();
    let input = io::stdin();

    println!("Enter firstname: ");
   input.read_line(&mut first_name);

    println!("Enter firstname: ");
    input.read_line(&mut last_name);

    format!("{} {}", first_name.trim(), last_name.trim())
}