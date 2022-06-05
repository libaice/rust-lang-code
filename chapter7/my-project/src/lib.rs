// default private
//
// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_whitelist() {}
//     }
// }
//
// pub fn eat_at_restaurant() {
//     crate::front_of_house::hosting::add_to_whitelist();
//     front_of_house::hosting::add_to_whitelist();
// }


// super call

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 相对路径
        super::serve_order();

        // 绝对路径
        crate::serve_order();
    }

    fn cook_order() {}
}


// pub struct  字段默认私有