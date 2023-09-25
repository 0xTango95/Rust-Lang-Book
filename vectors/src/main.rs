fn main() {
    println!("Hello, world!");

    // New empty vector
    let v: Vec<i32> = Vec::new();

    // Vector with initial values
    let v = vec![1, 2, 3];

    //  Updating a vector
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Reading Elements of Vectors
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
}
