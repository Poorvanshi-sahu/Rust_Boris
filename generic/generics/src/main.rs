// generics is a placeholder but for a type, not for value like functions

#[derive(Debug)]
struct DeliSandwich{

}

fn identity_int(value: i32) -> i32{
    value
}

fn identity_bool(value: bool) -> bool{
    value
}

fn identity<T>(value: T) -> T{
    value
}

fn make_tuple<T>(first: T, second: i32)-> (T, i32){
    (first, second)
}   

fn make_tuple2<T>(first: T, second: T)-> (T, T){
    (first, second)
}   

fn make_tuple3<T, U>(first: T, second: U)-> (T, U){
    (first, second)
}   

// generics in structs
#[derive(Debug)]
struct TreasureChest<T>{
    captain: String,
    treasure: T
}

// generics and impl block I

// only when the type is string
impl TreasureChest<String>{
    fn clean_treasure(&mut self){
        self.treasure = self.treasure.trim().to_string()
    }
}

impl TreasureChest<[&str; 3]>{
    fn amount_of_treasure(&self) -> usize{
        self.treasure.len()
    }
}

impl <T> TreasureChest<T>{
    fn capital_captain(&mut self){
        self.captain = self.captain.to_uppercase()
    }  

    fn calc_length(&self)->usize{
        self.treasure.len()
    }
}

fn main() {
    let ans = identity::<bool>(true);
    println!("{ans}");

    let ans2 = identity::<i32>(890);
    println!("{ans2}");

    let ans3 = identity::<i8>(12);
    println!("{ans3}");

    let ans4 = identity::<DeliSandwich>(DeliSandwich{});
    println!("{ans4:?}");

    // multiple generics
    let ans5 = make_tuple("pop", 90);
    println!("{ans5:?}");

    let ans6 = make_tuple2("aaa", "bbb");
    println!("{ans6:?}");

    let ans7 = make_tuple3("aaa", 67767);
    println!("{ans7:?}");

    // generics in structs
    let mut gold_chest = TreasureChest{
        captain: String::from("Poorvanshi"),
        treasure: 7898
    };

    gold_chest.capital_captain();
    println!("{:?}", gold_chest);

    let mut silver_chest = TreasureChest{
        captain: String::from("Royal"),
        treasure: String::from("People")
    };

    println!("{:?}", silver_chest);
    silver_chest.clean_treasure();

    let platinum_chest = TreasureChest{
        captain: String::from("platinum"),
        treasure: ["one", "two", "three"]
    };

    println!("{}",platinum_chest.amount_of_treasure());

    // println!("{:?}", gold_chest);
    println!("{:?}", silver_chest);    

    println!("{}", platinum_chest.calc_length());
}
