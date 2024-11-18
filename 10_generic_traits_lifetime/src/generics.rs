// generic functions:
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// generic data types
struct Point<T> {
    x: T,
    y: T,
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

// generics methods:
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// and specialization:
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// types does not have to match always:
struct Point2<X1, Y1> {
    x: X1,
    y: Y1,
}

// so X1, Y1 are declared after impl because they go with
// the struct definition and parameters X2 and Y2 are declared
// after mixup because they are only relevant to the method!
impl<X1, Y1> Point2<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    // let integer_and_float = Point { x: 5, y: 4.0 }; -> will not compile, datatypes does not match!

    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
