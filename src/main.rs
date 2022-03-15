mod rectangle_area_calculator;
mod fibonacci;
mod degree_converter;

use std::io;
use crate::{rectangle_area_calculator::Rectangle};

fn main() {
    println!("What function would you like to test?");
    println!("Type 'DEGREE' for a Fahrenheit/Celcius Converter");
    println!("Type 'FIB' for a Fibonacci Number Finder");

    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to Read Line");

    if user_input.trim().eq("DEGREE") {
        degree_converter::degree_converter();
    } else if user_input.trim().eq("FIB") {
        fibonacci::fibonacci_number();
    } else {
        println!("Did not recognize that function! {}", Rectangle{width:30, height:50}.area());
    }
}
