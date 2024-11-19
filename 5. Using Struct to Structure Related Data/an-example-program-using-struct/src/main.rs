#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };
    println!("The area of rect 1 is {}", area(&rect1));
    // println!("rect 1 is {rect1:?}");
    // println!("rect 1 is {rect1:#?}");
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
