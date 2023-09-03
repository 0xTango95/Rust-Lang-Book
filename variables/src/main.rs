// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// CONSTANTS

// fn main() {
//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");
// }

// Shadowing
// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {}", x);
// }

// DATATYPES

// fn main() {
//     // let guess: u32 = "42".parse().expect("Not a number!");
//     // println!("It is {guess}")

//     let x: i32 = -5;
//     println!("The value of x is: {}", x)

// }

// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

//     let z: f32 = x + y;
//     println!("The value of z is: {}", z);
// }

// fn main() {
//     let tup = (500, 6.4, 1);

//     let x = tup.0;
//     let y = tup.1;
//     let z = tup.2;

//     println!(
//         "The value of x is: {}, the value of y is {}, the value of z is {}",
//         x, y, z
//     )
// }

// // The Tuple Type
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let index = 0;

//     let element = a[index];

//     println!("The value of element is: {}", element);
// }

// Invalid Array Element Access
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
