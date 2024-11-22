fn main() {
    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // println!("{v:?}");

    // let v = vec![1, 2, 3];
    // println!("{v:?}");

    // let mut v: Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);
    //
    // println!("{v:?}");

    // let v = vec![1, 2, 3, 4, 5];
    // let third = v[2];
    // println!("The third element is {third}");
    //
    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element")
    // }

    // let v = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v[100]; // This cause an error
    // let does_not_exist: Option<&i32> = v.get(100);
    // match does_not_exist {
    //     Some(value) => println!("{value}"),
    //     None => println!("Index is out of range")
    // }

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    //
    // v.push(6);
    // println!("The first element is {first}");

    // let v = vec![100, 32, 57];
    // for n_ref in &v {
    //     let n_plus_one = *n_ref + 1;
    //     println!("{n_plus_one}");
    // }

    // let mut v = vec![100, 32, 57];
    // for n_ref in &mut v{
    //     *n_ref += 50;
    // }
    // println!("{v:?}");

    // let mut v = vec![String::from("Hello"), String::from("World")];
    // for n_ref in &mut v{
    //     // use dereference to change value on reference
    //     *n_ref = format!("{} Changed", n_ref);
    //     println!("{n_ref}");
    // }
    // println!("{v:?}");

    // #[derive(Debug)]
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     String(String),
    // }
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::String(String::from("Blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];
    // println!("{row:?}");

    // {
    //     let v = vec![1, 2, 3];
    // }
    // println!("{v}");

    // let mut v = vec![1, 2, 3];
    // let mut new: Vec<i32> = Vec::new();
    // for i in &mut v {
    //     new.push(*i);
    // }
    // println!("{v:?}");
    // println!("{new:?}");

    // v is set to mutable because we will save mutable reference to v on variable v2
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v2: Vec<&mut i32> = Vec::new();
    for i in &mut v{
        v2.push(i);
    }

    *v2[0] = 5;
    let a = *v2[0];
    let b = v[0];
    println!("{a}, {b}");
}
