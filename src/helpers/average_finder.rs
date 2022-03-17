use std::io; 

pub fn average_calculator() -> () {
    let mut input_array: [usize;5] = [0;5];
    for x in 0..5 {
        input_array[x] = gather_input((x+1).try_into().unwrap());
    }

    println!("The median is {}", find_median(&mut input_array));
    println!("The average is {}", find_average(&mut input_array));
}

fn gather_input(x: u8) -> usize {
    println!("({}/5) Type any number: ", x);

    let mut first = String::new(); 

    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read input");
    
    let first_num = first.trim().parse().expect("Only Integers Excepted!");

    first_num
}

fn find_median(arr: &mut [usize;5]) -> usize {
    arr.sort();
    arr[2]
} 

fn find_average(arr: &mut [usize;5]) -> usize {
    let mut avg: usize = 0;
    for x in 0..arr.len() {
        avg += arr[x]
    }
    avg / 5
} 