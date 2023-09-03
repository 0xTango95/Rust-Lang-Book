fn main() {
    let mut s = String::from("Hello This Is A String Of Words");

    let word = first_word(&s[..]);
    println!("The length of the first word in {} is {}", s, word);

    // s.clear(); // this empties the String, making it equal to ""

    let my_string_literal = "Hello Rustacean";
    let word_two = second_word(&my_string_literal[..]);
    println!("The second word is {}", word_two);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // iterate over the array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
}

fn second_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut first_index: usize = 0;
    let mut found_first: bool = false;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && !found_first {
            first_index = i + 1;
            found_first = true;
        } else if item == b' ' && found_first {
            return &s[first_index..i];
        }
    }

    &s[..]

    // or
    // if (found_first) {
    //     &s[first_index..s.len()]
    // } else {
    //     ""
    // }
}
