fn main() {
    let data = "initial contents"; // string slice

    let s = data.to_string(); // here we will allocate it on heap

    // the method also works on a literal directly:
    let s = "initial contents".to_string();
    // or just:
    let s = String::from("initial contents");

    // appending to a String:
    let mut s = String::from("Daniel");
    s.push_str(" World_1"); // it takes string slice so it will not take ownership

    println!("{s:?}");

    let mut s = String::from("lo");
    s.push('l');

    // concatenation with plus operator:
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + &s2;
    println!("Data together: {}", s3);

    // or easier using format! macro
    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = format!("{s1}{s2}");
    println!("Data together using format macro: {}", s3);

    /*
    WHY IN:
    let s3 = s1 + &s2;
    we have `s1` and `&s2`?
    BECAUSE:

    has to do with the signature of the method
    that’s called when we use the + operator.
    The + operator uses the `add`` method, whose
    signature looks something like this:

    fn add(self, s: &str) -> String {...





     */
}
