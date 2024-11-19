#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    // value_in_cents(Coin::Quarter(UsState::Alabama));
    //
    // let five = Some(5);
    // let six = plus_one(five);
    // let none = plus_one(None);
    // println!("Six: {six:?}, none: {none:?}");

    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     9 => remove_fancy_hat(),
    //     _ => reroll()
    // }
    // fn add_fancy_hat(){}
    // fn remove_fancy_hat(){}
    // fn reroll(){}

    let opt: Option<String> = Some(String::from("Hello"));
    match &opt {
        Some(s) => println!("Some: {s}"),
        None => println!("None!")
    }
    println!("{opt:?}"); // opt still exist because we just borrow opt by reference not by ownership

}

fn value_in_cents(coin: Coin) -> u8 {
    match coin{
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        Some(i) => Some(i + 1),
        None => None
    }
}
