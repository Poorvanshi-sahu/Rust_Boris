fn main() {
    if true{
        println!("This will be printed");
    }

    if false{
        println!("Not be printed");
    }

    even_or_odd(91);

    // match statment
    let val = true;

    let result = match val{
        true=>40,
        false=>90,
    };

    println!("result: {result}");

    // ***************************** underscore
    println!("underscore");

    let season: &str = "winter";

    match season{
        "winter" => println!("Brr, so cold"),
        "summer" => println!("Brr, so hot"),
        "autumn" => println!("rainy today"),
        _ => println!("anything"),
    }


    // match with multiple values
    let value:i32 = 189;

    match value{
        num if num % 2 == 0 => println!("num is even"),
        x if x % 2 != 0 => println!("num is odd") ,
        _ => unreachable!(),
    }


    // loop and break

    let mut seconds = 10;
    let mut min = 21;
    
    loop{
        if seconds == 0{
            println!("Breakoff");
            break;
        }
        println!("{seconds}");
        seconds -= 1;

    }

    // continue forcefully iterate the loop to next iteration
    // loop{
    //     if min<=0{
    //         println!("break");
    //         break
    //     }

    //     if min%2==0{
    //         println!("{min} is even");
    //         min-=3;
    //         continue;
    //     }

    //     println!("{min} is odd");
    //     min-=1;
    // }

    // ***************************** while
    while min>0{
        if min%2==0{
            println!("{min} is even");
            min-=3;
            continue;
        }

        println!("{min} is odd");
        min-=1;
    }

    println!("break!");

    


}

// if else
fn even_or_odd(number: i32){
    let result = if number % 2 ==0 {"even"} else {"odd"};
    println!("Number {number} is {result}");
}