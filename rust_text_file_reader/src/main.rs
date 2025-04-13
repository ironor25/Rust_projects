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



use std::{fs, io};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

//METHDO TO CHECK TYPE OF VAR IN RUST.
// use std::any::type_name;

// fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }


fn file(value:&String) -> File{
    let path = Path::new(value.trim());
    let display = path.display();
    println!("file saving to {}", display);
            let file = match File::create(&path) {
                Err(why) => panic!("Cant create file due to {}",why),
                Ok(file) => file,
                };
            return file;
        
        
}

    


fn read(value:&String)-> String {
    let mut content = String::new();
    let path = Path::new(value.trim());

    let mut file = match File::open(&path) {
        Err(why) => panic!("Cant create file due to {}",why),
        Ok(file) => file,
        
    };
    
    match file.read_to_string(&mut content){
        Err(why) => panic!("couldn't read: {}", why),
        Ok(_) => return  content,

    };

    
    
}



fn update(value:&String){
    let mut content:String = read(value);

    let mut input_text = String::new();
    println!("Enter some input: ");
    io::stdin().read_line(&mut input_text).expect("Failed to read!");
    content = content + &input_text;
    match file(value).write_all(content.as_bytes()) {
        Err(why) => panic!("Can't create file due to {}",why),
        Ok(_) => println!("your file updated"),
    }
         
    
}

fn create(value:&String){
    let mut input_text = String::new();
    println!("Enter some input: ");
    io::stdin().read_line(&mut input_text).expect("Failed to read!");

    match file(value).write_all(input_text.as_bytes()) {
        Err(why) => panic!("Can't create file due to {}",why),
        Ok(_) => println!("your file created"),
    }
}

fn delete(value:&String){
    let trimmed_value = value.trim();
    fs::remove_file(trimmed_value).expect("file not found");
    println!("\n {} file deleted", trimmed_value);

}

fn  parsing_in(input:&String)-> i32{
    let parsed_input:i32 = input.trim().parse().expect("not able to parse");
    return parsed_input;
}


fn main() {
    let mut input = String::new();
    let mut parse_int:i32 = 5 ;
    let mut value= String::new();
    
    while parse_int>0{
        println!("CRUD operations:\n 1: Create \n 2: Read \n 3:Update \n 4:Delete  \n 0: Exit");
        io::stdin().read_line(&mut input).expect("Failed to read!");
        // println!("{}",type_of(&input));
        
        parse_int = parsing_in( &input);
        match parse_int {
            1 => {
                println!("Enter file path");
                io::stdin().read_line(&mut value).expect("Failed to read!");
                create(&value);
                input = String::new();
                value = String::new();
                
            },
            2 => {
                println!("Enter file path");
                io::stdin().read_line(&mut value).expect("Failed to read!");
                println!("{}",read(&value)) ;
                input = String::new();
                value = String::new();
                
            },
            3 => {
                println!("Enter file path");
                io::stdin().read_line(&mut value).expect("Failed to read!");
                update(&value);
                input = String::new();
                value = String::new();
            },
            4 => {
                println!("Enter file path");
                io::stdin().read_line(&mut value).expect("Failed to read!");
           
                delete(&value);
                input = String::new();
                value = String::new();
            },
            0 => {
                parse_int = 0;
            },

            _ => {
                println!("Entered wrong choice!")
            }
            
        }
    }
    println!("Have a good time>")
}





