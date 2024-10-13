use crate::garden::vegetables::Asparagus;
pub mod garden; // include code in src/garden.rs

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    ::restaurant::eat_at_restaurant();
    ::restaurant::eat_at_restaurant_with_use();
}
