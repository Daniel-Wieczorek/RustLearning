fn main() {
    // Problem:
    // word what stores size of `s` is not in sync with s. So
    // once we invalide s word will be no longer valid. Programmer
    // will have to track if that's valid or not.
    let mut s = String::from("Hello world");
    let word = first_word(&s);
    s.clear();
    println!("Size: {word}");

    // Solution: string slice
    let mut s: String = String::from("Hello world");
    let hello = first_word_sliced(&s);
    println!("Sliced: {hello}");
    s.clear();
    // line below will then create compilation error:
    // println!("Sliced: {hello}");
    // Why?
    // In s.clear(); mutuable borrow occurs here. And before in `first_word_sliced` we had
    // inmutable refenrece . That's not allowed in rust!

    // That will generate panic. Idk why bounds are not checked here?
    let s = String::from("Hello world");
    let _out_of_bunds = &s[..99];
}

fn first_word(data: &String) -> usize {
    let bytes = data.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    data.len()
}

fn first_word_sliced(data: &String) -> &str {
    let bytes = data.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &data[0..i];
        }
    }
    &data[..]
}

// take slice as argument instead of `String!`
fn _first_word_sliced_as_arg(data: &str) -> &str {
    let bytes = data.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &data[0..i];
        }
    }
    &data[..]
}
