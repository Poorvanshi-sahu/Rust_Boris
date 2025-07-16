/* Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.
 
Refactor the function above to use the `match` statement
instead of if, else if, and else.
 
Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.
 
Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.
 
The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.
 
Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn main() {
    println!("Hello, world!");
    let ans = color_to_number("blue");
    println!("ans: {ans}");

    println!("fac: {}", factorial_rec(6));
    println!("not fac: {}", factorial_not_rec(6));
}

fn color_to_number(color: &str)-> i32{
    // if color == "red"{
    //     1
    // }else if color == "green"{
    //     2
    // }else if color == "blue"{
    //     3
    // }else{
    //     0
    // }

    match color{
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0,
    }
}

fn factorial_rec(num: i32)-> i32{
    if num == 0{
        return 1
    }
    num * factorial_rec(num-1)
}

fn factorial_not_rec(mut num: i32)-> i32{
    let mut ans: i32 = 1;
    while num != 0{
        ans = num * ans;
        num -= 1;
    }

    ans
}


