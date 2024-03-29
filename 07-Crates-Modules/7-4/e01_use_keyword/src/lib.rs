mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Absolute path with use
use crate::front_of_house::hosting;

// Relative path with use
// use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
