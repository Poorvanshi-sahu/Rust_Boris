use std::io;

fn main() {
    let string_slice = "popo uippohfj";
    let heap_string = String::from("Poorvanshi");

    // println!("{}", &string_slice[0..]);
    // println!("{}", &heap_string[0..]);

    // single quotes for character
    // double quotes for string
    let mut full_name = String::from("Poorvanshi");
    let last_name = "Sahu";

    full_name.push(' ');
    full_name.push_str(last_name);

    println!("{}", full_name);
    println!("{}", last_name);

    let one  = String::from("Poorvanshi_name");
    let two = String::from("Sahu");
    let no = 133;

    let formatted_one = format!("{0} {1} {0} {0} {2}", one, two, no);

    println!("{}", formatted_one);


    // ******************* methods
    let mut music_genre = "     grep, mut, pop, klj";
    println!("{}", music_genre.trim());
    println!("{}", music_genre.trim_end());
    println!("{}", music_genre.trim_start());

    music_genre = music_genre.trim();
    println!("{}", music_genre);

    println!("{}", music_genre.to_uppercase());
    println!("{}", music_genre.to_lowercase());

    println!("{}", music_genre.replace("r", "j"));
    let genres : Vec<&str> = music_genre.split(", ").collect();

    println!("{:?}", genres);

    let mut name = String::new();

    println!("What is your name?");

    match io::stdin().read_line(&mut name) {
        Ok(_) => println!("Everything is ok"),
        Err(message) => println!("Error: {}", message)
    } 

    // trim removing \n
    println!("Your name is {}", name.trim());

}
