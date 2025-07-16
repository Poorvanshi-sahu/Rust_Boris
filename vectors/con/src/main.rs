// vectors can grow and shrink at run time, they are stored in heap memory.

// vectors can implement debug trait, but not display trait

fn main() {
    // this is array
    let arr: [&str; 3] = ["hello", "guys", "kese ho"];
    println!("{:?}", arr);

    // ****************************************** this is vector
    // let vec: Vec<&str> = Vec::new();
    // let vec2 = Vec::<&str>::new();
    // println!("{:?}", vec);
    // println!("{:?}", vec2);
    // println!("{vec2:?}");

    // ******************************************  this is populated vector
    let mut vec: Vec<i32> = vec![9, 90, 78, 67];
    let vec2 = Vec::<&str>::new();
    println!("{:?}", vec);
    println!("{:?}", vec2);
    println!("{vec2:?}");

    vec.push(890);
    vec.push(129);
    println!("{:?}", vec);

    let rem = vec.remove(0);
    println!("{:?} rem: {}", vec, rem);

    vec.insert(0, 5008);
    println!("{:?}", vec);

    let last_elem = vec.pop();
    println!("{:?} {:?}", vec, last_elem);

    let pepperoni = String::from("pepperoni");
    let mushroom = String::from("mushroom");
    let sausage = String::from("sausage");

    let pizza_vec = vec![pepperoni, mushroom, sausage];
    // let mut new_var = pizza_vec;

    // let cop = &pizza_vec[0];

    // println!("{:?}", pizza_vec);
    // println!("{:?}", cop);

    // let pizza_slice = &pizza_vec[1..2];
    // println!("{:?}", pizza_slice);

    // get is bit safe to use, instead of directing accessing index as it returns option enum which handles none case as well
    // let res = pizza_vec.get(2);
    
    // match res {
    //     Some(val) => println!("{}", val),
    //     None => println!("Value not present at that index")
    // }

    // println!("{:?}", pizza_vec[2]);

    // let new_pizza_vec = &(&new_var)[1];

    // println!("{}", new_pizza_vec);
    
    // new_var.push(String::from("Olives"));


    // ******************************* writing vector element
    let mut topping_vector = vec![String::from("pepperoni"), String::from("mushroom"), String::from("sausage")];
    let one_elem = &mut topping_vector[1];

    one_elem.push_str(" and seasoning");

    println!("{:?}", topping_vector);


    // ******************************* vector capacity
    let mut seasons: Vec<&str> = Vec::with_capacity(2);
    seasons.push("pop");
    seasons.push("push");
    seasons.push("poooo");
    seasons.push("poppp");
    println!("len: {} capacity: {}", seasons.len(), seasons.capacity()); 

}
