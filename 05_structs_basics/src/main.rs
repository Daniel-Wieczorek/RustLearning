struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32,
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

#[derive(Debug)]
struct RectangleExtended {
    width: u64,
    height: u64,
}

impl RectangleExtended {
    fn area(&self) -> u64 {
        self.width * self.height
    }
}

// Below we have two associated methods for RectangleExtended
impl RectangleExtended {
    // can be called with width() like any other method
    fn _width(&self) -> bool {
        self.width > 0
    }

    fn _can_hold(&self, other: &RectangleExtended) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

//And one not associated method for RectangleExtended used as constructor
impl RectangleExtended {
    fn square(size: u64) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
    // Self is alias for what goes after `impl`
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("RootUser"),
        email: String::from("root@root.com"),
        sign_in_count: 1,
    };

    println! {"User email before change: {0}", user1.email};
    user1.email = String::from("other@email.com");
    println! {"User email before change: {0}", user1.email};

    // Using update syntax:
    let _user2 = User {
        email: String::from("another@email.com"),
        ..user1 // update from user1
    };

    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit like structs:
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    // using tuple:
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    println!("Debug information {rect1:#?}");
    // another way using dbg! macro:
    dbg!(&rect1);

    // method implementation
    let rect2 = RectangleExtended {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    let sq = RectangleExtended::square(300);
    println!(
        "The area of the sq rectangle is {} square pixels.",
        sq.area()
    );
}

// Return struct from function
fn _build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// use tuple
fn area_tuple(rectangle: (u64, u64)) -> u64 {
    rectangle.0 * rectangle.1
}

// use struct
fn area_struct(rectangle: &Rectangle) -> u64 {
    rectangle.width * rectangle.height
}
