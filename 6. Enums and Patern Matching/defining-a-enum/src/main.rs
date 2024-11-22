// // enum IpAddr {
// //     V4(String),
// //     V6(String),
// // }
// //
// // fn main() {
// //     let four = IpAddr::V4(String::from("127.0.0.1"));
// //     let six = IpAddr::V6(String::from("::1"));
// //
// //     route(four);
// //     route(six);
// // }
// //
// // fn route(ip_kind: IpAddr){
// //     match ip_kind {
// //         IpAddr::V4(ip) => println!("Ip: {ip}"),
// //         IpAddr::V6(ip )=> println!("Ip: {ip}"),
// //     }
// // }
// // enum Message {
// //     Quit,
// //     Move {x: u32, y: u32},
// //     Write(String),
// //     ChangeColor(i32, i32, i32)
// // }
// // impl Message {
// //     fn call(&self){
// //         match self {
// //             Message::Write(text) => println!("{text}"),
// //             _ => println!("Not String"),
// //         }
// //     }
// // }
// //
// // fn main() {
// //     let msg = Message::Write(String::from("Hello"));
// //     msg.call();
// // }
//
fn main() {
    let x: i8 = 5;
    // let y: Option<i8> = Some(5);
    let y: Option<i8> = None;
    // let sum = x + y; // cant add Option<i8> to i8

    let sum = match y {
        Some(val) => val + x,
        None => x,
    };
    println!("The sum x and y is {sum}");
}
