fn main() {
    println!("Hello, world!");
    print_lable_measurement(31, 'h');

    let y = {
        let x = 5;
        x + 1
    };
    println!("{y}");
    println!("{}", five());
    println!("{}", plus_one(6));
}

fn print_lable_measurement(value: u32, unit: char){
    println!("The measurement is {value}{unit}");
}

fn five()->u32{
    5
}

fn plus_one(value: u32)->u32{
    value + 1
}