use std::io;

pub fn area_calculator() -> () {
    let mut height = String::new();
    
    println!(
        "What is the height of your rectangle in ft?"
    );

    io::stdin()
        .read_line(&mut height)
        .expect("Failed to Read Line");

    let height = height.trim().parse().expect("Please type a Number!");

    let mut width = String::new();
    
    println!(
        "What is the width of your rectangle in ft?"
    );

    io::stdin()
        .read_line(&mut width)
        .expect("Failed to Read Line");

    let width = width.trim().parse().expect("Please type a Number!");

    let rect = Rectangle { width: width, height: height};

    println!("The area of your rectangle is {}ft squared", rect.area());
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
     fn area(&self) -> u32 {
        self.height * self.width
    }

    // fn can_hold(&self, rect: &Rectangle) -> bool {
    //     let can_hold: bool = true;
    //     if self.width < rect.width {
    //         return false;
    //     }
    //     if self.height < rect.height {
    //         return false;
    //     }
    //     can_hold
    // }

    // fn square(size: u32) -> Rectangle {
    //     Rectangle {
    //         width: size,
    //         height: size,
    //     }
    // }
}