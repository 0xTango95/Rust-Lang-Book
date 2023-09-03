// Declare a module named `front_of_house` (not implemented in this snippet)
mod front_of_house;

// Define a function to serve orders
fn serve_order() {}

// Declare a module named `back_of_house`
mod back_of_house {
    // Define a public enum for appetizers
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // Define a public struct for breakfast
    pub struct Breakfast {
        pub toast: String,      // Public field for toast type
        seasonal_fruit: String, // Private field for seasonal fruit
    }

    // Implement methods for the Breakfast struct
    impl Breakfast {
        // Public constructor function for a fall breakfast
        pub fn fall(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("apples"),
            }
        }
    }

    // Private function to fix incorrect orders
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // Call the `serve_order` function from the parent scope
    }

    // Private function to cook orders
    fn cook_order() {}
}

// Using the restaurant (commented out in the original snippet)
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();
//
//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// Import the `hosting` module so it can be used in `eat_at_restaurant`
pub use crate::front_of_house::hosting;

// Public function to simulate eating at the restaurant
pub fn eat_at_restaurant() {
    // Create appetizer orders
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Create a breakfast order using the fall constructor
    let mut meal = back_of_house::Breakfast::fall("Rye");

    // Change the toast type
    meal.toast = String::from("Marble Rye");

    // Print the toast type
    println!("I'd like {} toast please", meal.toast);

    // Uncommenting the next line would cause an error because `seasonal_fruit` is private
    // meal.seasonal_fruit = String::from("figs");

    // Add to the waitlist using the imported `hosting` module
    hosting::add_to_waitlist();
}
