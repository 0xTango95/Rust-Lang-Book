use std::io;

fn main() {
    // Prompting the user to enter the desired Fibonacci number.
    println!("What Fibonacci number would you like?");

    let mut n = String::new(); // Creating a mutable variable to store the user input.

    io::stdin()
        .read_line(&mut n) // Reading the user input and storing it in the 'n' variable.
        .expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num, // Parsing the user input to an unsigned 32-bit integer.
        Err(_) => {
            panic!("Please input a valid positive number."); // Handling the case of invalid input with a panic.
        }
    };

    // Calling the 'nth_fibonacci' function to calculate the desired Fibonacci number.
    let fibonacci_number = nth_fibonacci(n);

    // Displaying the result to the user.
    println!("The {} fibonacci number is {}", n, fibonacci_number);
}

// Function to calculate the nth Fibonacci number.
// 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144
fn nth_fibonacci(n: u32) -> u32 {
    // Base cases: The first two Fibonacci numbers are 0 and 1.
    if n == 1 {
        return 1; // If n is 1, return the first Fibonacci number (1).
    } else if n == 0 {
        return 0; // If n is 0, return the second Fibonacci number (0).
    }

    // Recursive case: If n is greater than 1, calculate the Fibonacci number using recursion.
    nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
}
