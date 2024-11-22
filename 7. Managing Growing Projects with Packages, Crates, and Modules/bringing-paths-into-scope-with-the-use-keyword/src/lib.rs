mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){}
    }
}

// This scope is accessible from outside
pub use crate::front_of_house::hosting;

mod customer {
    // hosting invalid here
    pub fn eat_at_restaurant(){
        super::hosting::add_to_waitlist();
    }
}
