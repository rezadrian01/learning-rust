use std::io;

fn main() {
    // // Scalar type
    // // Integer
    // let x: u32 = 1000;
    // println!("Value of x is {x}");

    // // Floating point
    // let x = 2.1; // f64;
    // let y: f32 = 3.1;
    // println!("Value of x is {x}");
    // println!("Value of y is {y}");

    // // Char
    // let initial = 'a';
    // println!("{initial}");

    // // Compound type
    // let tup:(u32, f64, u8) = (100, 3.14, 2);
    // println!("{:?}", tup);
    // println!("{}", tup.1);

    // let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    // println!("{:?}", months);

    // let a: [u32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5];
    // println!("{:?}", a);

    let a = [1, 2, 3, 4, 5];
    println!("Please enter a index of an array");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read input");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered not a number");

    let element = a[index];
    println!("The value of the element at index {} is {}", index, element);
}