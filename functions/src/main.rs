// fn main() {
//     print_labeled_measurement(5, 'h');
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

fn main() {
    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    // println!("The value of y is: {y}");

    // let x = five();

    // println!("The value of x is: {x}");

    // let x = plus_one(-2);

    // println!("The value of x is: {x}");

    // expression_example();

    // println!(
    //     " Is 3 greater than five? Answer: {}",
    //     greater_than_number(3)
    // )

    // // Handling Multiple Conditions with else if
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // // Using if in a let Statement
    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // println!("The value of number is: {number}");

    // This will return an error because the if and else arms have value types that are incompatible
    // let condition = true;

    // let number = if condition { 5 } else { "six" };

    // println!("The value of number is: {number}");

    // //Repetition with Loops
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // // Loop Labels to Disambiguate Between Multiple Loops
    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // // Conditional Loops with while

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // // Looping Through a Collection with for
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // // if you changed the definition of the a array to have four elements
    // // but forgot to update the condition to while index < 4, the code would panic
    // // As a more concise alternative, you can use a for loop
    // let a = [10, 20, 30, 40, 50];

    // for element in a.iter().rev() {
    //     println!("the value is: {element}");
    // }

    // // A for loop and rev, to reverse the range
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");
}

// fn five() -> u32 {
//     5
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn expression_example() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is {}", y);
// }

// fn greater_than_number(x: i32) -> bool {
//     if x > 5 {
//         true
//     } else {
//         false
//     }
// }
