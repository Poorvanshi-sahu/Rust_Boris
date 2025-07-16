fn main() {
    // let mut current_meal = String::new();
    // add_flour(&mut current_meal);
    // show_my_meal(&current_meal);

    // // mutliple mutable refernce
    // let car = String::from("blue");
    // let red_car = &car;
    // let green_car = &car;

    // println!("{car} {red_car} {green_car} {}", &car);


    // mutliple immutable refernce 
    // let mut car = String::from("blue");
    // let red_car = &mut car;
    // let green_car = &mut car;

    // println!("{green_car} {}", &car);

    // mutliple immutable refernce 
    let mut car = String::from("blue");
    let red_car = &car;
    let green_car = red_car;

    println!("{green_car} {red_car}");

    // ***************************** Dangling references
    
}

// fn add_flour(mut meal: &mut String){
//     meal.push_str("Add flour");
// }

// fn show_my_meal(meal: &String){
//     println!("Meals: {meal}");
// }