fn main() {
    // Condition
    // let number = 4;

    // if number < 5{
    //     println!("condition was true");
    // }else{
    //     println!("condition was false");
    // }

    // if number != 0 {
    //     println!("number was something other than zero")
    // }


    // if number % 4 == 0 {
    //     println!("Number is divisible by 4");
    // }else if number % 3 == 0{
    //     println!("Number is divisible by 3");
    // }else if number % 2 == 0{
    //     println!("Number is divisible by 2");
    // }else{
    //     println!("Number is not divisible by 4, 3, or 2");
    // }

    // let condition: bool = true;
    // let number = if condition {5} else {6};

    // println!("The value of number is: {number}");

    // Loop
    // loop {
    //     println!("again!");
    // }

    // let mut counter = 0;
    // let result = loop {
    //     counter += 1;
    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };
    // println!("The result is {result}");

    let mut counter = 0;
    'counting_up: loop{
        println!("Counter is: {}", counter);
        let mut remaining: u32 = 10;
        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if counter == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        counter += 1;
    };

    // let mut number = 3;
    // while number != 0{
    //     println!("Number = {number}");
    //     number -= 1;
    // }
    // println!("LIFTOFF");

    // Bad practice
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < a.len(){
    //     println!("The value is: {}", a[index]);
    //     index += 1;
    // }

    // Best practice
    // for element in a{
    //     println!("The value is: {element}");
    // }

    // for number in (1..4).rev(){
    //     println!("{number}");
    // }
    // println!("LIFTOFF");

}
