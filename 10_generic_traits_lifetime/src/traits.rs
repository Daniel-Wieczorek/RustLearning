#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt::Debug;
use std::fmt::Display;

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.username)
    }
}

pub trait Summary {
    fn summarize_author(&self) -> String; // no default implementation

    fn summarize(&self) -> String {
        // default implementation already there
        format!("Default Article author is: {}.", self.summarize_author())
    }
}

// traits as parameters
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// but we can make it more generic (trait bound syntax? )
pub fn notify_generic<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// with two same types:
pub fn notify_generic_two_same_types<T: Summary>(item: &T, item2: &T) {}

// and different ones
pub fn notify_generic_two_different_types<T: Summary + Display>(item: &T, item2: &T) {}

// -------------------------
// we can combine traits as arguments as follows:
fn some_function<T: Summary + Display, U: Display + Debug>(t: &T, u: &U) -> i32 {
    // impl.
    return 0;
}

// but that one can be written in a simpler form, see below:
fn some_function_but_with_where<T, U>(t: &T, u: &U) -> i32
where
    T: Summary + Display,
    U: Display + Debug,
{
    // impl.
    return 0;
}
// -------------------------
// returning trait:
fn returns_summarize() -> impl Summary {
    // return any type what is implementing Summary trait
    Tweet {
        username: String::from("I am returned trait from function!"),
        content: String::from("Bla bla bla"),
        reply: false,
        retweet: false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Our T must implement Disaply and Partial order
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("@dani"),
        content: String::from("Hello world in Tweet"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        author: String::from("Daniel"),
        headline: String::from("World is coming!!!"),
        content: String::from("Hello world in Article"),
    };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    notify(&article);
    notify_generic(&article);

    println!("{}", returns_summarize().summarize())
}
