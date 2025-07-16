use std::collections::{HashMap, HashSet};

fn main() {
    let mut menu: HashMap<String, f64> = HashMap::new();
    let mut country_capitals = HashMap::<&str, &str>::new();

    menu.insert(String::from("Poorvanshi"), 25.0);
    menu.insert(String::from("Riu"), 14.0);
    menu.insert(String::from("Ami"), 12.0);

    println!("{:#?}", menu);
    println!("{:#?}", country_capitals);


    // conversion to hashmap
    let data = [("abc", 90), ("pop", 12), ("mmm", 14)];

    let data_hash = HashMap::from(data);

    // let new_data_hash= data_hash;

    println!("{:#?}", data_hash);

    // println!("{:?}", data);


    // ownership
    let mut my_hash: HashMap<&str, &str> = HashMap::new();
    let one = String::from("one");
    let two = String::from("two" );
    my_hash.insert(&one, &two);

    my_hash.insert("three", "four");
    println!("{:?}", my_hash);

    let value = my_hash.get("myValue").copied().unwrap_or("nothing found");

    match my_hash.get("myValue"){
        Some(_)=> println!("yes working"),
        None => println!("err: nothing")
    }

    println!("{:?}", value);

    // to make entry only when it is not present
      my_hash.entry("nine").or_insert("ten");
      my_hash.entry("one").or_insert("two");

    println!("{:#?}", my_hash);


    // ****************************** Hashset
    let mut hashset = HashSet::new();

    hashset.insert("one");
    hashset.insert("two");

    println!("{:?}", hashset);

    hashset.remove("two");

    println!("{:?}", hashset);


    // ****************************** Hashset operations
    let mut concert_queue = HashSet::new();
    let mut movie_queue = HashSet::new();

    concert_queue.insert("Boris");
    concert_queue.insert("Melissa");
    println!("{:#?}", concert_queue);

    movie_queue.insert("Phil");
    movie_queue.insert("non");
    // movie_queue.insert("Boris");
    println!("{:#?}", movie_queue);

    let union_set = concert_queue.union(&movie_queue);
    println!("{:?}", union_set);

    println!("{:?}", concert_queue.difference(&movie_queue));
    println!("{:?}", movie_queue.difference(&concert_queue));

    // present in either one not both
    println!("{:?}", concert_queue.symmetric_difference(&movie_queue));
    println!("{:?}", movie_queue.symmetric_difference(&concert_queue));

    // anything common
    println!("{:?}", concert_queue.is_disjoint(&movie_queue));
    println!("{:?}", movie_queue.is_disjoint(&concert_queue));

    // value of one in other
    println!("{:?}", concert_queue.is_subset(&movie_queue));
    println!("{:?}", movie_queue.is_subset(&concert_queue));

}
