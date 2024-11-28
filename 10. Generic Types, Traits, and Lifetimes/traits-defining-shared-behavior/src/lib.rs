use std::fmt::Display;
use std::iter::Sum;

pub trait Summary {
    // fn summarize(&self) -> String;
    // Default implementation

    // fn summarize(&self) -> String{
    //      String::from("(Read more...)")
    // }
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String{
        format!("(Read more from {}...)", self.summarize_author())
    }

}

pub struct NewsArticle{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle{
    fn summarize(&self) -> String {
        format!("{}, by {}, ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}

impl Summary for Tweet{
    // fn summarize(&self) -> String {
    //     format!("{}: {}", self.username, self.content)
    // }

    fn summarize_author(&self) -> String {
        format!("@{}",self.username)
    }
}

// Traits as Parameters
pub fn notify(item: &impl Summary){
    println!("Breaking news! {}", item.summarize());
}

// Traits Bound Syntax
pub fn notify<T: Summary>(item: T){
    println!("Breaking news! {}", item.summarize());
}

// pub fn notify(item1: &impl Summary, item2: &impl Summary) item1 and item2 can be different type

// pub fn notify<T: Summary>(item1: T, item2: T) item1 and item2 must be the same type

// Multiple Traits Bound
// pub fn notify (item: &(impl Summary + Display))>
// pub fn notify<T: Summary + Display> (item: &T)

// Clearer Trait Bounds with where clause
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug
// {
//     unimplemented!()
// }

// Returning that implements traits
fn return_summarizable() -> impl Summary{
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    }
}

// We cannot return different types even if they have the same property implementation.
// fn return_summarizable() -> impl Summary{
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship"),
//             location: String::from("Pittsburgh PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from("\
//             The Pittsburgh Penguins once again are the best\
//             hockey team in the NHL")
//         }
//     }else {
//         Tweet{
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably know, people"
//             ),
//             reply: false,
//             retweet: false
//         }
//     }
// }

// Using Trait Bounds to Conditionaly Implement Methods
struct Pair<T>{
    x: T,
    y: T
}

impl <T> Pair<T>{
    fn new(x: T, y: T) -> self{
        Pair{x, y}
    }
}

impl <T: Display + PartialOrd> Pair<T>{
    fn cmp_display(&self) {
        if self.x > self.y{
            println!("The largest number is {}", self.x);
        }else{
            println!("The largest number is {}", self.y)
        }
    }
}