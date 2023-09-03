struct User {
    username: &str,
    email: &str,
    active: bool,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: "alai@rustamania.com",
        username: "alai",
        active: true,
        sign_in_count: 1,
    };

    println!("User's email is {}", user1.email);

    user1.username = "AlaiY";

    println!("User's name is {}", user1.username);

    // Can also use ..user1 to use the same values.
    //   let user2 = User {
    //     email: String::from("test@testymctest.com"),
    //     ..user1
    // };

    // create a new User instance in user2
    // We set a new value for email but otherwise use the same values from user1
    // let user2 = build_user(
    //     String::from("test@testymctest.com"),
    //     String::from("mcTester"),
    // );

    // println!("User2 email: {}, username: {}", user2.email, user2.username);

    // let inactive_user2 = User {
    //     active: false,
    //     ..user2
    // };

    // println!(
    //     "Inactive User2 Info - active: {}, username: {}",
    //     inactive_user2.active, inactive_user2.username
    // )
}

// Using Tuple Structs Without Named Fields to Create Different Types
fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

// build_user function that returns a User instance with the given email and username
fn build_user(email: &str, username: &str) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
