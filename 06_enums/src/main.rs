enum IpAddrKind {
    V4(String),
    V6(String),
    V8(u8, u8, u8, u8, u8),
}

// Enum can be created with many different subtypes
// without need to create separate structs to store it.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move message by {x} and {y}"),
            Message::Write(text) => println!("Written message: {text}"),
            Message::ChangeColor(_r, _g, _b) => println!("Color changed"),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- and others
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

// Most known impl of enum in std is Option<T> what is:
enum CustomOption<T> {
    None,
    Some(T),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn match_roll(dice_roll: &i32) {
    match dice_roll {
        3 => _add_fancy_hat(),
        7 => _remove_fancy_hat(),
        _ => (),
    }
}

fn _add_fancy_hat() {}
fn _remove_fancy_hat() {}
fn _reroll() {}

fn main() {
    println!("Hello, world!");

    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("127.0.0.2"));
    let home = IpAddrKind::V8(127, 0, 0, 0, 1);

    let m = Message::Write(String::from("Communication init"));
    m.call();

    // using Option -> here it can change the type as we have `T` in `Some(T)`
    let some_number = Some(5);
    let some_char = Some('e');

    // and here type can not be deduced only from `None` so type is used before
    let absent_number: Option<i32> = None;

    // How will it help, see:
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // That will generate compiler error what is expected
    // as we want to add i8 and Option<i8> types:
    // let sum = x + y;
    value_in_cents(Coin::Quarter(UsState::Alaska));

    let five = Some(5);
    let six = plus_one(five);
    // let None = plus_one(None);

    // if let syntax: instead of:
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The max val is {max}"),
        _ => (),
    }

    // we can write using `if let`
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The max val from if let!");
    }
}
