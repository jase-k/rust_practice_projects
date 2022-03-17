use std::io;

pub fn fibonacci_number() -> () {
    println!(
        "What number of the Fibonacci Sequence would you like to know? (Type an integer below)"
    );

    let mut fib_num = String::new();

    io::stdin()
        .read_line(&mut fib_num)
        .expect("Failed to Read Line");

    let fib_num = fib_num.trim().parse().expect("Please type a Number!");

    let fib_total = calculate_fib(fib_num);
    println!("Your Number {}", fib_total);
}

fn calculate_fib(num: u32) -> u32 {
    if num < 2 {
        return 1;
    }
    let mut fib_num1 = 0;
    let mut fib_num2 = 1;
    let mut fib_total = 1;

    for _x in 0..num {
        fib_total = fib_num1 + fib_num2;
        fib_num1 = fib_num2;
        fib_num2 = fib_total;
    }

    fib_total
}
