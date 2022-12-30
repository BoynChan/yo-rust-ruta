pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {
            super::serving::take_over();
        }
    }

    mod serving {
        pub fn take_over() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(totas: &str) -> Breakfast {
            Breakfast {
                toast: String::from(totas),
                seasonal_fruit: String::from(""),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("totas");
    meal.toast = String::from("Wheat");
}
