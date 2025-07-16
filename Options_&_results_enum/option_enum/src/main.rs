fn main() {
    let a = Option::Some("hello");
    let b = Option::Some(90);
    let c = Option::Some(String::from("oops!"));

    let d = Option::<i32>::Some(34);

    let e: Option<&str> = Option::None;

    let f: Option<&str> = Option::Some("12");

    println!("{:?} {:?} {:?} {:?} {:?} {:?} ", a, b, c, d, e, f);

    let musical_instrument = [
        String::from("guitar"),
        String::from("Drums"),
        String::from("Base")
    ];

    let bass = musical_instrument.get(0);
    let bass2 = musical_instrument.get(890);

    println!("bass: {:?}", bass);
    println!("bass2: {:?}", bass2);
    println!("{:?}", musical_instrument[0]);

    play_instrument(bass);

    // ************************  unwrap and expect panic on err

    // println!("{:?}",bass.unwrap());

    // bass2.unwrap();  //panic and terminate

    // bass2.expect("something went wrong");

    // ************************ match

    // match bass{
    //     Option::Some(instrument) => println!("Here playing {:?}", bass),
    //     Option::None => println!("playing None"),
    //     _ => println!("none")
    // }
}


fn play_instrument(option_string: Option<&String>){
    match option_string{
        Option::Some(instrument) => println!("Here playing {:?}", instrument),
        Option::None => println!("playing None"),
        _ => println!("none")
    }
}

