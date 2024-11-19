#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
impl Rectangle {
    fn other_method(&self){
        println!("This method come from other impl block");
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let square1 = Rectangle::square(3);
    // println!("The area of rect1 is {}", rect1.area());
    // if rect1.width() {
    //     println!("The rect1 has a nonzero width; it is {}", rect1.width);
    // }
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("Square: {:#?}", square1);
    square1.other_method();
}


// Exercise
// struct Book {
//     title: String,
//     author: String,
// }
// impl Book {
//     fn description(&self) -> String {
//         format!("Judul: {}, Penulis: {}", self.title, self.author)
//     }
// }
// fn main(){
//     let book1 = Book{
//         title: String::from("Atomic Habits"),
//         author: String::from("James Clear")
//     };
//     println!("{}", book1.description());
// }

// struct Circle {
//     radius: f64
// }
// impl Circle {
//     fn area(&self) -> f64{
//         3.14 * self.radius * self.radius
//     }
// }
//
// fn main(){
//     let circle1 = Circle {
//         radius: 10.0
//     };
//     println!("{}", circle1.area());
// }

// struct Person {
//     name: String,
//     age: u8
// }
// struct Address {
//     street: String,
//     city: String,
//     zip_code: u32
// }
//
// impl Person{
//     fn print_address(&self, address: &Address) {
//         println!("{} tinggal di {}, {} dengan kode post {}", self.name, address.street, address.city, address.zip_code);
//     }
// }
//
// fn main(){
//     let person1 = Person {
//         name: String::from("Reza"),
//         age: 20
//     };
//     let address1 = Address {
//         street: String::from("Dinoyo"),
//         city: String::from("Malang"),
//         zip_code: 65144
//     };
//     person1.print_address(&address1);
// }

// struct Rectangle {
//     width: u32,
//     height: u32
// }
// impl Rectangle {
//     fn get_width(&self) -> u32 {
//         self.width
//     }
//     fn get_height(&self) -> u32 {
//         self.height
//     }
//     fn set_width(&mut self, width: u32) {
//         self.width = width;
//     }
//     fn set_height(&mut self, height: u32) {
//         self.height = height;
//     }
// }
//
// fn main(){
//     let mut rectangle1 = Rectangle {
//         width: 30,
//         height: 50
//     };
//     println!("Panjang persegi: {}, tinggi persegi: {}", rectangle1.width, rectangle1.height);
//     rectangle1.set_width(10);
//     rectangle1.set_height(20);
//     println!("Panjang persegi: {}, tinggi persegi: {}", rectangle1.width, rectangle1.height);
//
// }

// struct Rectangle {
//     width: f64,
//     height: f64
// }
// struct Circle {
//     radius: f64
// }
//
// impl Rectangle {
//     fn area(&self) -> f64 {
//         self.width * self.height
//     }
//     fn can_hold(&self, other: &Circle) -> bool {
//         self.area() > other.area()
//     }
// }
// impl Circle {
//     fn area(&self) -> f64 {
//         3.14 * self.radius * self.radius
//     }
// }
// fn main() {
//
// }

// struct Square {
//     side: u32
// }
//
// impl Square{
//     fn new(side: u32) -> Self{
//         Self{
//             side
//         }
//     }
// }
//
// fn main() {
//     let square1 = Square::new(5);
//     println!("{}", square1.side);
// }