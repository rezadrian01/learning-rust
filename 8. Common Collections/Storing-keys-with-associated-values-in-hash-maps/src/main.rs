use std::collections::HashMap;

fn main() {
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // println!("{scores:?}");

    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    //
    // let team_name = String::from("Blue");
    //
    // // get method return an Option<&V> if there's no value for that key, get will return None
    // // copied method for copying value from hashmap not borrowing
    // // unwrap_or method for set score to zero if scores doesn't have entry for the key
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    //
    // println!("{score}");

    // let mut scores = HashMap::new();
    //
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yello"), 50);
    //
    // for(key, value) in &scores{
    //     println!("{key}: {value}");
    // }
    // println!("{scores:?}");

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    //
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    //
    // // field_name and field_value is no longer valid here.

    // Overwriting a value
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    // println!("{scores:?}");

    // // Adding a key and value only if a key isn't present
    // let mut scores: HashMap<String, i32> = HashMap::new();
    // scores.insert(String::from("Blue"), 10);
    //
    // // we get mutable reference here so we can change that value directly
    // let temp = scores.entry(String::from("Yellow")).or_insert(50);
    // // scores.entry(String::from("Blue")).or_insert(50);
    //
    // // *temp = 100;
    // println!("{temp}");
    // println!("{scores:?}");

    let text = "hello world wonderfull world";
    let mut map = HashMap::new();

    for word in text.split_whitespace(){
        // we have mutable reference for the current word
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{map:?}");

}
