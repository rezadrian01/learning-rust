fn  main(){
    // let s1 = String::from("Hello");
    // let length = calculate_length(&s1);
    // println!("The length of '{s1}' is {length}");

    // let mut s = String::from("Hello");
    // change(&mut s);
    // println!("{s}");

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;

    // let r1 = & s;
    // let r2 = & s;
    // let r3 = &mut s;
    // println!("{}, {}, {}", r1, r2, r3);

    // let r1 = & s;
    // let r2 = & s;
    // println!("{}, {}", r1, r2);
    // // r1 and r2 will not be used after this point
    //
    // let r3 = &mut s;
    // r3.push_str(", world");
    // println!("{}", r3);

    let reference_to_nothing = dangle();
}
// fn calculate_length(s: &String) -> usize{ // s is a reference to a String being send
//     s.len()
// }// Here s goes out of scope. But because it does not have ownership of what it refer to, it is no dropper.

// fn change(some_string: &mut String){
//     some_string.push_str(", World");
// }

fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!