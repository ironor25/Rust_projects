//VERSION 1
// use std::fs::File;
// use std::io::{self,Read};


// fn main() ->  io::Result<()>  {

// //   let mut path =  String::new();
// //   let mut content = String::new();
// //   
  
// //   }

//     let mut file = File::open("hq.txt")?;
//     let mut content = String::new();

//     file.read_to_string(&mut content)?;

//     println!("{}",content);

//     Ok(())

use std::collections::btree_map::Values;
use std::{io, path};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Display, Path};


fn file(value:String) -> File{
    let path = Path::new(&value);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("Cant create file due to {}",why),
        Ok(file) => file,
        
    };
    return file;

}


fn read(value:String)-> String {
    let mut content = String::new();
    
    return match file(value).read_to_string(&mut content)  {
        Err(why) => panic!("not able to read file {}",why),
        Ok(_) => content,
    } 
    }



fn Update(value:String){
    let mut content:String = read(value);

    let mut input_text = String::new();
    println!("Enter some input: ");
    io::stdin().read_line(&mut input_text).expect("Failed to read!");
    content = content + &input_text;
    
    match file(value).write_all(input_text.as_bytes()) {
        Err(why) => panic!("Can't create file due to {}",why),
        Ok(_) => println!("your file created"),
    }
    
    
    
}

fn Create(&value:str){
    let mut input_text = String::new();
    println!("Enter some input: ");
    io::stdin().read_line(&mut input_text).expect("Failed to read!");

    match file(value).write_all(input_text.as_bytes()) {
        Err(why) => panic!("Can't create file due to {}",why),
        Ok(_) => println!("your file created"),
    }
}



fn  parsing_in(input:String)-> i32{
    let parsed_input:i32 = input.trim().parse().expect("not able to parse");
    return parsed_input;
}

fn main() {
    let mut input = String::new();
    let mut parse_in:i32 = 5 ;
    let mut value= String::new();

    while parse_in>0{
        println!("Option for Calculations:\n 1: Create \n 2: Read \n 3:Update \n 4:Delete  \n 0: Exit");
        io::stdin().read_line(&mut input).expect("Failed to read!");
        
        println!("Enter file path");
        io::stdin().read_line(&mut value).expect("Failed to read!");
        
        parse_in = parsing_in( input.clone());

        match parse_in {
            1 => {
                Create(value);
            },
            2 => {},
            3 => {},
            4 => {},
            0 => {},
            _ => {}
            
        }
    }
}





