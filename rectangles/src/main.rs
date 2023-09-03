// Option 1
// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1, height1)
//     );
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// Option2
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     // println!(
//     //     "The area of the rectangle is {} square pixels.\n",
//     //     area(&rect1)
//     // );

//     // This won't work.
//     // println!("rect1 is {}", rect1);

//     // This will work using:
//     // #[derive(Debug)]
//     // struct Rectangle {
//     // println!("rect1 is {:?} \n", rect1);
//     // //  larger structs use {:#?}
//     // println!("rect1 is {:#?}", rect1);

//     // Example where we’re interested in the value that gets assigned to the width field,
//     // as well as the value of the whole struct in rect1
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };
//     // dbg! returns ownership of the expression’s value
//     // The dbg! macro can be really helpful when you’re trying to figure out what your code is doing!
//     dbg!(&rect1);
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// Method Syntax

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
