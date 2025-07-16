fn main() {
    let name = String::from("Poorvanshi sahu");
    let name_reference = &name;
    let name_slice = &name[0..5];

    let last_name_slice = &name[11..15];

    // length of a string slice means the length of its bytes not its character
    println!("{name_reference}\n{name_slice}\n{last_name_slice} \nlen: {}", last_name_slice.len());

    // syntactic shortcut
    let my_name = String::from("Poorvanshi sahu");
    let my_name2 = "riu";
    let string_slice = &my_name[..5];
    let string_slice_2 = &my_name[5..];
    let string_slice_3 = &my_name[..];
    println!("{} {} {}", string_slice, string_slice_2, string_slice_3);

    // string slices as function parameter
    print_name(&my_name);
    print_name(&my_name2);

    // Array slices
    let arr = [12,13,14,15,16];
    let arr_slice = &arr[1..2];
    let arr_slice2 = &arr[1..5];

    println!("arr_slice: {arr_slice:?} arr_slice2: {arr_slice2:?}");

    // ***************************** Deref coercion
    let value = [10, 20, 30, 50, 70, 80];
    let value_reference = &value;
    let array_slice = &value[5..];

    print_array(value_reference);
    print_array(array_slice);

    // Mutably array slices
    let mut my_array = [10,20,405,06,60];
    let my_array_slice = &mut my_array[1..4];

    my_array_slice[0]=1000;
    println!("my array slice: {:?}", my_array_slice);
    println!("my array: {:?}", my_array);
}

fn print_name(name: &str){
    println!("{} is my name", name)
}

fn print_array(arr: &[i32]){
    println!("{arr:?}")
}