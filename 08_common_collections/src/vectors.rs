#![allow(dead_code)]
#![allow(unused_variables)]

fn main() {
    let mut v1: Vec<i32> = Vec::new();

    // or using macro without specifying type:
    let v2 = vec![1, 2, 3];

    // adding elements:
    v1.push(5);
    v1.push(6);

    // accessing elements:
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let v = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v[100]; // That will panic at runtime - no bounds checking!
    let does_not_exist = v.get(100); // That will

    /*
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);
    println!("The first element is: {first}");

    4 |     let first = &v[0];
      |                  - immutable borrow occurs here
    5 |
    6 |     v.push(6);
      |     ^^^^^^^^^ mutable borrow occurs here
    7 |
    8 |     println!("The first element is: {first}");
      |                                     ------- immutable borrow later used here
         */

    // for loop in vec

    // using immutable reference
    for i in &v {
        println!("{i}");
    }

    // using mutuable reference:
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    // vectors can store values from the same type but that can be worked out using enum:
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.12),
        SpreadSheetCell::Text(String::from("blue")),
    ];
}
