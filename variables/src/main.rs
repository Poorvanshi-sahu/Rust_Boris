#![allow(unused_variables)]

// for constants
#![allow(dead_code)]

// GLOBAL CONST VARIABLE
const GLOBAL: i32 = 890;

// TYPE ALIAS
type Meters = i32;


fn main() {
    //  by default variable are immutable

    // let mut apples: Meters = 50;
    // let oranges: i32 = 23;
    // let _fruits: i32 = apples + oranges; // underscore to make it as unused

    // apples = 90;

    // println!("I have {0} apples, {1} oranges.", apples, oranges);

    // println!("value of const global {}", GLOBAL)


    // **************************directives 
    // #[allow(unused_variables)]
    let apples: Meters = 50;
    let oranges: i32 = 23;

    // println!("I have {0} apples, {1} oranges.", apples, oranges);


}
