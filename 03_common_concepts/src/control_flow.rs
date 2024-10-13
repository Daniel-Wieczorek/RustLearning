fn main() {
    // Simple if statement
    let number = 3;
    if number < 5 {
        // note that `if` is an expression
        println!("Condition true");
    } else {
        println!("Condition false");
    }

    // if - elseif - else leader
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // if with let
    let condition = true;
    let number = if condition { 5 } else { 6 }; // Note: types must mach here!
    println!("The value of number based on condition is: {number}");

    // Loops -> `loop`
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result from `loop` is {result}");

    // breaking nested loops with label:
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    // basic while loop
    let mut number = 3;
    while number != 0 {
        println!("{number} number in while loop");
        number -= 1;
    }

    // basic `for`` loop for iterating over array:
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value of element in for loop is: {element}");
    }

    // iterate up using `for` loop
    for element in 1..5 {
        println!("Iterate up in for loop: {element}");
    }

    // iterate down using `for` loop
    for element in (1..5).rev() {
        println!("Iterate down in for loop: {element}");
    }
}
