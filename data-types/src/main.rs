use std::ops::Range;

fn main() {
    // let eight_bits: i8 = -127;  // consumes only 8 bits or 1 byte of integer

    // usize for unsigned
    let a: usize = 123;

    // isize for signed
    let b: isize = 145;

    println!("{a}, {b}");

    // tab
    println!("\t Once upon a time");

    // " is valid character do not ignore it
    println!("\" whatss up \"");

    // let filePath : &str = "C:\\Document\\path";
    let file_path : &str = r"C:\Document\path";

    println!("{file_path}");

    //*****************************************************methods********************************************************* */

    let value: i32 = -12;

    println!("{}", value.abs());

    let e_string: &str = "           th           ";

    println!("{}", e_string.trim());

    println!("{}", value.pow(2));

    //*****************************************************floating point************************************************** */
    let pi: f64 = 3.141534624567419545;

    println!("{pi}");
    println!("{}", pi.floor());
    println!("{}", pi.ceil());
    println!("{pi:.5}");
    println!("{:.5}", pi);

    // ****************************************************casting**********************************************************
    let value_as_i32 = 50;
    let value_as_i8 = value_as_i32 as i8;
    let value_as_u8: u8 = value_as_i32 as u8;

    let value_as_float = 14.3213;
    let value_as_int = value_as_float as i8;

    println!("value as i32: {value_as_i32}, value as i8: {value_as_i8}, value as u8: {value_as_u8}, value as float: {value_as_float}, value as int: {value_as_int}");

    // ****************************************************math**************************************************************

    // ****************************************************augmented assignment operator************************************************************
    let mut year = 2025;
    year += 1;

    println!("{year}");

    // *****************************************************boolean**********************************************************************************
    let is_handsome = true;
    let is_silly = false;
    let age: i32 = -30;

    println!("{} {}", age.is_positive(), age.is_negative());

    // **************************************************** Boolean inversion***********************************************************************
    println!("{} {}", !age.is_positive(), age.is_negative());

    //****************************************************Equality and equality operator**********************************************************
    println!("****************************************************Equality and equality operator**********************************************************");
    println!("{}", 2==90);
    println!("{}", 90!=89);

    //************************************************************* && ************************************************************************** */

    //************************************************************* // ************************************************************************** */

    //************************************************************* character type ************************************************************** */
    let first_char = 'L';
    let second_key = 'S';
    println!("{first_char} {} {}", first_char.is_alphabetic(), first_char.is_uppercase());

    // scalar types and compund types
    // scalar type single value: int, boolean, float, char, string
    // compound type: collection of one or more value

    // *********************************************************** Array ***************************************************************
    let array: [i32; 5] = [1, 2, 3, 56, 5];
    let ar : [i32;2 ] = [65, 45];
    let a : [f64;0] = [];

    println!("{}", array.len());

    // ******Debug trait******
    println!("{:?}", array);

    // to print pretty
    println!("{array:#?}");

    //*********************************************************** dbg! macro ***********************************************************
    dbg!(array);

    //*********************************************************** tuple ***********************************************************
    // array has homogeneous element, tuple can have different element
    // can not implement display trait i.e println!("{}")
    // can implement debug trait

    let employee: (&str, i32, &str) = ("abc", 90, "abc@gmail.com");

    // let name = employee.0;
    // let age = employee.1;
    // let gmail = employee.2;

    let (name, age, gmail) = employee;
    println!("Name :{name}, Age:{age}, Gmail: {gmail}");
    println!("{employee:?}");

    //*********************************************************** range and range iteration *********************************************************** 
    let month_days = 1..31;
    println!("{month_days:?}");

    let month_days = 1..=31;
    println!("{month_days:?}");

    for day in month_days{
        println!("{day}");
    }

    for elem in array{
        println!("elem: {elem}");
    }

    let letters = 'a'..'z';

    for letter in letters{
        println!("letter: {letter}");
    }

     //***************************************** Generic is pleaceholder for future type or expected type *************************************************
     // yet to clear

     //*****************************************project */

     println!("Here project starts"); 

     let var = 13_37;

     let new_val = var as i16;

     println!("new_val: {new_val}");

     let float_var = 1.24345334;

     println!("{:.3}", float_var);

     let with_milk = true;
     let with_sugar = true;

     let is_my_type_of_coffee = with_milk && with_sugar;

     let is_acceptable_coffee = with_milk || with_sugar;

     println!("is_my_type_of_coffee: {is_my_type_of_coffee}");

     println!("is_acceptable_coffee: {is_acceptable_coffee}");

     let arr: [i8; 4] = [12, 44, 24, 103];

     println!("{arr:?}");

     let tup = (12,"opop","v");

     println!("{tup:?}");



}
