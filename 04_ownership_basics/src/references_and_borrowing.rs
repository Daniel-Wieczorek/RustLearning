fn main() {
    let s1 = String::from("hello");
    let len = calculate_len(&s1);

    println!("The length of '{s1}' is {len}.");

    // we can pass data via reference:
    let mut s1 = String::from("hello");
    change_mutable(&mut s1);
    println!("change_mutable of s1 is `{s1}`");

    // we can also do that but way above is preferred
    let s1 = String::from("hello");
    let s1 = change_mutable_with_ownership_exchange(s1);
    println!("change_mutable of s1 is `{s1}`");

    // handling of dangling references:
    let _s = String::from("Hello");
}

fn calculate_len(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

// That will fail to compile as we do not own reference thus can change it!
/*
fn change_immutable(some_string: &String) {
    some_string.push_str(", world");
}
*/

fn change_mutable(some_string: &mut String) {
    some_string.push_str(", mutuable world");
}

fn change_mutable_with_ownership_exchange(mut some_string: String) -> String {
    some_string.push_str(", mutuable world");
    some_string
}

/*
// handle dangling references:
fn dangle() -> &String {
    let s = String::from("Hello");
    &s // we return reference to the String s
    // and here, just below s goes out of scope so dangling
    // reference will not be created and compiler will report
    // error!
}
*/
