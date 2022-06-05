mod front_of_house {
    pub mod hosting {
        pub fn add_to_whitelist(){}
    }
}

// 绝对
// use crate::front_of_house::hosting;

// 相对
use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_whitelist();
    hosting::add_to_whitelist();
    hosting::add_to_whitelist();
}