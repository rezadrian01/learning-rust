struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    // let mut user1 = User{
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1
    // };
    // // user1.email = String::from("anotheremail@example.com");
    // // println!("{}", user1.email);
    // let user2 = User{
    //     email: String::from("another@example.com"),
    //     ..user1 // move username field ownership from user1 to user2
    // };
    // println!("{}", user2.username);
    // println!("{}", user2.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

// fn build_user(email: String, username: String) -> User {
//     User {
//         active: true,
//         email,
//         username,
//         sign_in_count: 1
//     }
// }
