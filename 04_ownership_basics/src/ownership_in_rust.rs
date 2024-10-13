fn main() {
    let s1 = String::from("hello");
    let _s2 = s1; // that will move content so s1 will be invalidate!

    // println!("{s1}, world!"); -> that will create compile time error as s1 in invalid already

    // cloning as deep copy:
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // stack only data: copy
    let x = 5;

    /*
       we don’t have a call to clone, but x is still
       valid and wasn’t moved into y. Why?
       Because:
       For basic types and some other types where
       we create data on stack this operation is not expensive
       and such data will be just deep copied.
    */
    let y = x;
    println!("x = {x}, y = {y}");

    // ownership and functions
    {
        let s = String::from("Hello"); // s comes into scope
        takes_ownership(s); // here we move `s` into a function so it will not be valid anymore
                            // ...
        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved, nothing
      // special happens.

    // return values and scope:
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1

        let s2 = String::from("Hello"); // s2 comes into scope

        let _s3 = takes_ownership_and_gives_it_back(s2); // s2 is moved into
                                                         // takes_and_gives_back, which also
                                                         // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    {
        let s1 = String::from("hello");
        let (s2, len) = return_as_touple(s1);
        println!("The length of '{s2}' is {len}.");
    }
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("takes_ownership prints: {some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_int: i32) {
    // // some_integer comes into scope
    println!("makes_copy prints: {some_int}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope
    some_string
    // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_ownership_and_gives_it_back(some_string: String) -> String {
    // some_string comes into
    // scope
    some_string // some_string is returned and moves out to the calling function
}

// We pass s by moving it so we need to return it back - here using touple
// but that can be done using references too
fn return_as_touple(s: String) -> (String, usize) {
    let length = s.len(); // len() returns string length
    (s, length)
}
