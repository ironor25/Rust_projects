use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;

fn main(){
    let mut input = String::new();
    let mut content = String::new();
    println!("Enter path to create file: ");
    io::stdin().read_line(&mut input).expect("Failed to read!");
    // println!("Enter content for fiel:");
    // io::stdin().read_line(&mut content).expect("Failed to read!");

    let path = Path::new(input.trim());
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Cant create file due to {}",why),
        Ok(file) => file,
        
    };
    match file.read_to_string(&mut content){
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!(" contains:{}", content),
    }
    }
    // match file.write_all(content.as_bytes()) {
    //     Err(why)=> panic!("file not created. bcz : {}",why),
    //     Ok(_) => println!("File created succesfully.")        
    // }
