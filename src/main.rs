mod helpers;
use std::io;

fn main() {
    println!("What function would you like to test?");
    println!("Type 0 for a Fahrenheit/Celcius Converter");
    println!("Type 1 for a Fibonacci Number Finder");
    println!("Type 2 for a Rectangle Area Calculator");
    println!("Type 3 for finding the average");

    let mut vector: Vec<fn()> = Vec::new();
        vector.push(helpers::degree_converter::degree_converter as fn());
        vector.push(helpers::fibonacci::fibonacci_number as fn());
        vector.push(helpers::rectangle_area_calculator::area_calculator as fn());
        vector.push(helpers::average_finder::average_calculator as fn());

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to Read Line");

    let input_number: usize = user_input.trim().parse().expect("Please Enter a Number!");

    match vector.get(input_number) {
        Some(funct) => funct(),
        None => println!("Sorry that's out of my capabilities"),
    };
}

// println!("Did not recognize that function! {}", Rectangle{width:30, height:50}.area());