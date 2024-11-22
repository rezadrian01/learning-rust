mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }
}

pub fn eat_at_restaurant (){
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order(){}

mod back_of_house{
    pub struct Breakfast {
        pub toast: String,
        // Only accessible from current module and child module
        seasonal_fruit: String,
    }
    impl Breakfast{
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // If enum set to public, then all value in enum is public too
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order(){
        cook_order();
        // Refer to parent of current module
        super::deliver_order();
    }
    fn cook_order(){}
}