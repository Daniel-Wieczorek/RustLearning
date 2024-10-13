fn main() {
    // Variables
    // let x = 5; -> that will not compile
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");

    // Const
    const THREE_HOURS_IN_SEC: u32 = 60 * 60 * 3;
    println!("The value of cost var is: {THREE_HOURS_IN_SEC}");

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The value of x is: {x}");

    // Primitive data types:
    let _decimal = 98_222; // `_` can be separator
    let _decimal_type = 57u8; // decimal with type
    let _hex = 0xff;
    let _binary = 0b1111_0000;
    let _byte = b'A';
    let _float = 2.0; // f64 by default
    let _boolean = true;
    let _heart_eyed_cat = '😻'; // single character, 4 bytes

    // Compound types
    // 1. Tuple
    let _tup_1: (i32, f64, u8) = (500, 6.4, 1);
    let _tup_2 = (100, 200, 300);

    // Acccessing elements:
    let (_x, _y, _z) = _tup_1; // here we are `destructing tuple!
    println!("The value of x is {_x} and y is {_y}");
    let _first_element = _tup_2.0;
    let _second_element = _tup_2.1;
    let _third_element = _tup_2.2;

    // 2. Array
    // Creation with just data
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // another way of creation, with data type and num of elements
    let _array_cnt: [i32; 5] = [1, 2, 3, 4, 5];

    // Create array of 5 elements with value of each = `3`
    let _same_value_array = [3; 5];
    let _valid_element = _same_value_array[0];
    // let _invalid_element = _same_value_array[10]; -> here we will have info about runtime panic and code will not compile!

    // usage of function:
    print_labeled_measurement(5, 'h');

    // statement and expression:
    let statement = {
        let x = 3;
        x + 1 // expression does not have `;`, otherwise these are statements!
              // to return value we need it as expression so without `;`
    };
    println!("The value of statement is: {statement}");

    // However that will not compile - statements does not return
    // values and `let y = 6` is statement:
    // let x = (let y = 6);
    let _c = 5 + 6; // here 5+6 is expression in this statement

    // get returned value from function
    let x = five();
    println!("The value of returned x is: {x}");
    let x = plus_one(5);
    println!("The value of x in plus_one is: {x}");
    let x = plus_one_with_return(5);
    println!("The value of x in plus_one_with_return is: {x}");
}

// simple function
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measuremnet is: {value}{unit_label}");
}

// function with return
fn five() -> i32 {
    5
}

// function with return and param
fn plus_one(x: i32) -> i32 {
    x + 1
}

// function with return and param
fn plus_one_with_return(x: i32) -> i32 {
    return x + 1;
}
