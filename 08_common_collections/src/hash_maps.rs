use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // accessing values:
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // for loop
    for (key, value) in &scores {
        println!("{key}, {value}");
    }

    // ownership:
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value); // both values will be moved here so become invalid

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{scores:?}");

    // adding a key and value only if a key is not present
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50); // it will be NOT updated, we have blue already
    scores.entry(String::from("Yellow")).or_insert(10); // it will be added as we do not have 10 for yellow
    println!("{scores:?}");

    // update value based on old value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let cnt = map.entry(word).or_insert(0);
        *cnt += 1;
    }

    println!("{map:?}");
}
