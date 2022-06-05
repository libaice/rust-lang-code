mod back_of_house {
    use crate::back_of_house;

    pub struct Breakfast {
        pub toast: String,
        season_fruit: String,
    }

    pub enum Appetizer {
        Soup,
        Solad,
    }


    pub fn eat_at_restaurant() {
        let order1 = back_of_house::Appetizer::Soup;
        let order2 = back_of_house::Appetizer::Solad;
    }


    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                season_fruit: String::from("Peaches"),
            }
        }
    }
}


pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!(" I would like {} toast please ", meal.toast);
}


