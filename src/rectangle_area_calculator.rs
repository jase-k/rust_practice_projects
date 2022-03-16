#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(&self, rect: &Rectangle) -> bool {
        let can_hold: bool = true;
        if self.width < rect.width {
            return false;
        }
        if self.height < rect.height {
            return false;
        }
        can_hold
    }

    pub fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// fn main() {
//     println!("Hello, world!");
//     // let width1 = 30;
//     // let height1 = 50;
//     let rectangle1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     let rect2 = Rectangle {
//         width: 35,
//         height: 12,
//     };

//     // let area = area(&rectangle1);

//     dbg!(&Rectangle::square(5));

//     println!("The area is {}", rectangle1.area());

//     println!("Can rect1 hold rect2? {}", rectangle1.can_hold(&rect2));
// }
