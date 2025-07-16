// result enum => is for modeling success vs err
// option enum => is for seeing avalialability


// returning result type from function
fn divide(numerator: f64, denominator: f64)-> Result<f64, String>{
    if denominator == 0.0 {
        Err("cannot divide by zero".to_string())
    }else{
        Ok(numerator / denominator)
    }
}

fn main() {
    // let ok: Result<i32, &str> = Result::Ok(5);
    // let disaster: Result<i8, &str> = Result::Err("Something went wrong");

    // println!("{:?}", ok);
    // println!("{:?}", disaster);

    let text = "50";
    let text_as_number = text.parse::<i32>();

    println!("{:?}", text_as_number);

    let text3 = "ok fine!";
    let text_as_number2 = text3.parse::<i32>();

    println!("{:?}", text_as_number2);

    let result = divide(10.0, 20.0);
    

    // match result{
    //     Ok(calculation) => println!("{}", calculation),
    //     Err(message) => println!("{}", message)
    // }

    println!("{}", result.is_ok());
    println!("{}", result.is_err());

    let s = 90;
    
}
