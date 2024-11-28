use std::fs::File;
use std::{fs, io};
use std::io::{ErrorKind, Read};

fn main() {
    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result{
    //     Ok(file) => file,
    //     Err(error) => match error.kind(){
    //         ErrorKind::NotFound => match File::create("hello.txt"){
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file {e:?}")
    //         },
    //         other_error => panic!("Problem opening the file {other_error:?}")
    //     }
    // };

    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error|{
    //     if error.kind() == ErrorKind::NotFound{
    //         File::create("hello.txt").unwrap_or_else(|error|{
    //             panic!("Problem creating the file {error:?}");
    //         })
    //     }else{
    //         panic!("Problem opening the file {error:?}");
    //     }
    // });

    // unwrap method will return the value if Ok and create panic! when Err
    // let greeting_file = File::open("hello.txt").unwrap();
    // println!("{greeting_file:?}");

    // expect method has same behavior with unwrap method, but with expect method you can define your own custom error msg
    let greeting_file = File::open("hello.txt").expect("hello.txt should ne included in this project");

}

// fn read_username_from_file() -> Result<String, io::Error>{
//     let username_file_result = File::open("hello.txt");
//
//     let mut username_file = match username_file_result{
//         Ok(file) => file,
//         Err(e) => return Err(e)
//     };
//
//     let mut username = String::new();
//     match username_file_result.read_to_string(&mut username){
//         Ok(_) => Ok(username),
//         Err(e) => Err(e)
//     };
// }

// ? operator, this operator will return result if operation return Ok, and call Err when operation return Err
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// ? operator can chain to other method
// fn read_username_from_file() -> Result<String, io::Error>{
//     let mut username = String::new();
//     File::open("hello.txt")?.read_to_string(&mut username)?;
//     Ok(username)
// }

fn read_username_from_file() -> Result<String, io::Error>{
    fs::read_to_string("hello.txt")
}

// ? operator can only use when the function has return type of Result, Option, or another type that implements FromResidual.
fn last_char_of_first_line(text: &str) -> Option<char>{
    text.lines().next()?.chars().last()
}