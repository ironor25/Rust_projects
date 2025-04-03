// 1. Task: Command-Line Calculator
// Concepts: Structs, Functions, Pattern Matching

// Create a simple command-line calculator that can perform basic operations like addition, subtraction, multiplication, and division.

// Implement a Calculator struct to store the current result.

// Use functions for each operation (add, subtract, etc.).

// Use pattern matching to handle different operations from the user's input.

use std::io;

fn parsing(x:String) -> i32{
    let x1:i32 = x.trim().parse().expect("enter valid no.");
    
    return x1;
}
fn add(x:i32,y:i32) ->  i32{
    let addition = x+y;
    return addition;
}
fn subtract(x:i32,y:i32) ->  i32{
    let sub = x-y;
    return sub;
}

fn multiply(x:i32,y:i32) ->  i32{
    let mul = x*y;
    return mul;
}

fn divide(x:i32,y:i32) ->  i32{
    let div = x/y;
    return div;
}


fn main() {
        struct Calculator{
         res:i32,
        }
        let mut input = String::new();
        let mut parse_in :i32 = 5;
        let mut x = String::new();
        let mut y= String::new();
        let mut values = Calculator{
            res:0,
        };

        while parse_in>0 {
         
            println!("Option for Calculations:\n 1: Addition \n 2: Subtraction \n 3: Multiplication \n 4: Division \n 0: Exit");
            io::stdin().read_line(&mut input).expect("Failed to read!");
            
            println!("Enter first value: ");
            io::stdin().read_line(&mut x).expect("Failed to read!");
            println!("Enter Second value: ");
            io::stdin().read_line(&mut y).expect("Failed to read!");
            parse_in = parsing(input.clone());
            
            match parse_in {
                1 => {
                    values.res = add( parsing(x.clone()),parsing(y.clone()));
                    println!("{}",values.res);},
                2 => {values.res =subtract(parsing(x.clone()),parsing(y.clone()));
                    println!("{}",values.res);},
                3 => {values.res =divide(parsing(x.clone()),parsing(y.clone()));
                    println!("{}",values.res);},
                4 => {values.res =multiply(parsing(x.clone()),parsing(y.clone()));
                    println!("{}",values.res);},
                0 => parse_in = 0,
                _ => {
                    println!("oops choosen wrong thing");
                    continue;
                },
            
            };


        }
        println!("thanks for calculating");

        
}
