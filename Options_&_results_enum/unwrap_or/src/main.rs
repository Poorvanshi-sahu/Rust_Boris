// building option from scratch
enum Option{
    Some(i32),
    None
}

impl Option{
    fn unwrap(self)-> i32{
        match self{
            Option::Some(value) => value,
            Option::None => panic!("SOMETHING WENT WRONG")
        }
    }

    fn unwrap_or(self, fallback_Value: i32)->i32{
        match self{
            Option::Some(value) => value,
            Option::None => fallback_Value
        }
    }
}

// nuances of heap method on string
fn operation(great_success: bool)-> Result<&'static str, &'static str>{
    if great_success{
        Ok("Success")
    }else{
        Err("Something is wrong")
    }
}

fn main() {
    // let present_value = Some(13);
    // let empty_value:Option<i32> = None;

    // println!("{:?}", present_value);
    // println!("{:?}", empty_value);

    // println!("{}", present_value.unwrap_or(0));
    // println!("{}", empty_value.unwrap_or(0));

    // building option from scratch
    let one = Option::Some(21);
    let two = Option::None;

    // one.unwrap();
    // two.unwrap();
    println!("{}",one.unwrap_or(890));
    println!("{}", two.unwrap_or(1289));


    let my_result = operation(false);
    let content = match my_result {
        Ok(message) => message,
        Err(err) => err,
    };

    println!("{}", my_result.unwrap_or("yes"));
}
