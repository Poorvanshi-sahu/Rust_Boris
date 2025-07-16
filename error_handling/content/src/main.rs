// use std::fs::File;
use std::{fs, io};
use std::io::{stdin, Read};

fn main() {
    // process::exit(0);
    // println!("print");

    println!("Some status message");
    eprintln!("Some error message");

    // let file = File::open("story.txt");
    // let res = match file {
    //     Ok(data) => data,
    //     Err(err)=> {
    //         eprintln!("something went wrong");
    //         process::exit(1);
    //     }
    // };
 
    // println!("{:?}", res);

    // ********************************** read file
    let read_file_res = read_file();

    match read_file_res {
        Ok(content) => println!("content: {}", content.trim()),
        Err(error)=> println!("error: {}", error)
    }
    
}


fn read_file()-> Result<String, io::Error>{
    let mut input = String::new();

   stdin().read_line(&mut input)?;
    // let user_requested_file = stdin().read_line(&mut input)?;


    // if let Err(err) = user_requested_file{
    //     return  Err(err);
    // }

    // let mut file_content = String::new();
    // File::open(&input.trim())?.read_to_string(&mut file_content)?;
    // let mut res = file?;
    // let mut res = match file {
    //     Ok(data) => data,
    //     Err(err)=> {
    //         return  Err(err);
    //     }
    // };

    
    // let read_operation = res.read_to_string(&mut file_content)?;

    // if let Err(err) = read_operation {
    //     return  Err(err);
    // }

    // Ok(file_content)

    // ***************** alternate way
    fs::read_to_string(input.trim())
}