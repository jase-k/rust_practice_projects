mod degree_converter;
mod fibonacci;
mod rectangle_area_calculator;

use crate::rectangle_area_calculator::Rectangle;
use std::io;

fn main() {
    println!("What function would you like to test?");
    println!("Type 0 for a Fahrenheit/Celcius Converter");
    println!("Type 1 for a Fibonacci Number Finder");
    let vector = vec[
        degree_converter::degree_converter,
        fibonacci::fibonacci_number,
    ];

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