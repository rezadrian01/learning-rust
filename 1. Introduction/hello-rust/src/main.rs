
// use ferris_says::say;
// use std::io::{stdout, BufWriter};
fn main() {
    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(&message, width, &mut writer).unwrap();
    let num1 = 5;
    let num2 = 10;
    let sum = num1 + num2;
    println!("{}", sum);
    if 5 < 2 {
        println!("True condition");
    }else{
        println!("False condition");
    }
    let mut counter = 0;
    while counter < 5{
        println!("{}", counter);
        counter += 1;
    }
    for i in 0..10{
        println!("{}", i);
    }
}
